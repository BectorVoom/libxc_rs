//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 903/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk903(t2951: f64, t7884: f64, t1181: f64, t1816: f64, t2970: f64, t2972: f64, t2974: f64, t2986: f64, t555: f64, t5862: f64, t7831: f64, t7835: f64, t7837: f64, t7842: f64, t7843: f64, t7851: f64, t7852: f64, t7857: f64, t7861: f64, t7866: f64, t7868: f64, t7874: f64, t7879: f64, t7881: f64) -> (f64, f64, f64) {
    let t7885 = t7884 * t2951;
    let t7887 = t1181 * t1816;
    let t7889 = -3.0_f64 / 32.0_f64 * t7831 * t2951 - t2970 * t7835 * t7837 / 12.0_f64 + t7842 * t2972 * t7843 / 16.0_f64 - t7851 - t2970 * t7852 * t2974 / 24.0_f64 - t2970 * t2972 * t7857 / 24.0_f64 - t2970 * t2972 * t7861 / 48.0_f64 - 7.0_f64 / 144.0_f64 * t7866 * t7868 * t7843 - t5862 / 64.0_f64 - t555 * t2986 * t7874 / 32.0_f64 - t7879 + t7881 / 96.0_f64 + 7.0_f64 / 32.0_f64 * t7885 + t7887 / 96.0_f64;
    (t7885, t7887, t7889)
}
