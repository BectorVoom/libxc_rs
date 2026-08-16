//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1426/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1426<F: Float>(t11478: F, t9681: F, t2893: F, t4501: F, t2851: F, t2889: F, t2858: F, t14770: F, t26118: F, t2829: F, t2834: F, t2838: F, t30790: F, t30793: F, t30796: F, t30807: F, t30826: F, t30829: F, t3661: F, t3688: F, t7637: F, t7806: F, t7811: F, t9549: F, t9625: F, t9766: F) -> (F, F, F, F, F) {
    let t30871 = t11478 * t9681;
    let t30880 = t4501 * t2893;
    let t30881 = t2851 * t30880;
    let t30891 = t4501 * t2889;
    let t30892 = t2858 * t30891;
    let t30895 = t2858 * t30880;
    let t30902 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t7811 * t30871 - F::cast_from(1600.0_f64) / F::cast_from(3.0_f64) * t9549 * t30790 + F::cast_from(1600.0_f64) / F::cast_from(3.0_f64) * t9549 * t30793 - F::cast_from(352.0_f64) / F::cast_from(3.0_f64) * t7806 * t30796 + F::cast_from(64.0_f64) / F::cast_from(81.0_f64) * t3661 * t30881 + F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t2829 * t30807 + F::cast_from(2520.0_f64) * t26118 * t9625 * t9766 + F::cast_from(64.0_f64) / F::cast_from(27.0_f64) * t3688 * t30881 - F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t2834 * t30892 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t2838 * t30895 - F::cast_from(448.0_f64) / F::cast_from(27.0_f64) * t14770 * t30826 - F::cast_from(224.0_f64) / F::cast_from(9.0_f64) * t7637 * t30829;
    (t30871, t30891, t30892, t30895, t30902)
}
