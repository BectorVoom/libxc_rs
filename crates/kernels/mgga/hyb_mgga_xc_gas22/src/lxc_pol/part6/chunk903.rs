//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 903/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk903<F: Float>(t2951: F, t7884: F, t1181: F, t1816: F, t2970: F, t2972: F, t2974: F, t2986: F, t555: F, t5862: F, t7831: F, t7835: F, t7837: F, t7842: F, t7843: F, t7851: F, t7852: F, t7857: F, t7861: F, t7866: F, t7868: F, t7874: F, t7879: F, t7881: F) -> (F, F, F) {
    let t7885 = t7884 * t2951;
    let t7887 = t1181 * t1816;
    let t7889 = -F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t7831 * t2951 - t2970 * t7835 * t7837 / F::cast_from(12.0_f64) + t7842 * t2972 * t7843 / F::cast_from(16.0_f64) - t7851 - t2970 * t7852 * t2974 / F::cast_from(24.0_f64) - t2970 * t2972 * t7857 / F::cast_from(24.0_f64) - t2970 * t2972 * t7861 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t7866 * t7868 * t7843 - t5862 / F::cast_from(64.0_f64) - t555 * t2986 * t7874 / F::cast_from(32.0_f64) - t7879 + t7881 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(32.0_f64) * t7885 + t7887 / F::cast_from(96.0_f64);
    (t7885, t7887, t7889)
}
