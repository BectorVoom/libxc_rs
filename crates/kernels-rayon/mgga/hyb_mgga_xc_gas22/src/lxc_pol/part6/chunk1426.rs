//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1426/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1426(t11478: f64, t9681: f64, t2893: f64, t4501: f64, t2851: f64, t2889: f64, t2858: f64, t14770: f64, t26118: f64, t2829: f64, t2834: f64, t2838: f64, t30790: f64, t30793: f64, t30796: f64, t30807: f64, t30826: f64, t30829: f64, t3661: f64, t3688: f64, t7637: f64, t7806: f64, t7811: f64, t9549: f64, t9625: f64, t9766: f64) -> (f64, f64, f64, f64, f64) {
    let t30871 = t11478 * t9681;
    let t30880 = t4501 * t2893;
    let t30881 = t2851 * t30880;
    let t30891 = t4501 * t2889;
    let t30892 = t2858 * t30891;
    let t30895 = t2858 * t30880;
    let t30902 = 32.0_f64 / 9.0_f64 * t7811 * t30871 - 1600.0_f64 / 3.0_f64 * t9549 * t30790 + 1600.0_f64 / 3.0_f64 * t9549 * t30793 - 352.0_f64 / 3.0_f64 * t7806 * t30796 + 64.0_f64 / 81.0_f64 * t3661 * t30881 + 32.0_f64 / 27.0_f64 * t2829 * t30807 + 2520.0_f64 * t26118 * t9625 * t9766 + 64.0_f64 / 27.0_f64 * t3688 * t30881 - 32.0_f64 / 9.0_f64 * t2834 * t30892 + 32.0_f64 / 9.0_f64 * t2838 * t30895 - 448.0_f64 / 27.0_f64 * t14770 * t30826 - 224.0_f64 / 9.0_f64 * t7637 * t30829;
    (t30871, t30891, t30892, t30895, t30902)
}
