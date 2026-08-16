//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1090/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1090(t6669: f64, t847: f64, t10629: f64, t3321: f64, t8923: f64, t3357: f64, t8906: f64, t4114: f64, t809: f64, t2234: f64, t1347: f64, t3352: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10630 = t6669 * t847;
    let t10631 = t10629 * t10630;
    let t10635 = 4.0_f64 * t8923 * t3321;
    let t10637 = 0.32163958997385070134e2_f64 * t8906 * t3357;
    let t10638 = t4114 * t809;
    let t10640 = 6.0_f64 * t2234 * t10638;
    let t10641 = t1347 * t3352;
    (t10631, t10635, t10637, t10638, t10640, t10641)
}
