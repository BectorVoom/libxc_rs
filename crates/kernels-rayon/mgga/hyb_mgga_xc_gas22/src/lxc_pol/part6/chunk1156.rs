//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1156/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1156(t547: f64, t5885: f64, t1801: f64, t1896: f64, t3023: f64, t580: f64, t1909: f64, t6012: f64, t1900: f64, t1905: f64, t17: f64, t7940: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19700 = t547 * t5885;
    let t19706 = 1.0_f64 / t1896 / t1801;
    let t19735 = t3023 * t580;
    let t19737 = t6012 * t1909;
    let t19739 = t6012 * t1900;
    let t19744 = t6012 * t1905;
    let t19746 = t7940 * t17;
    (t19700, t19706, t19735, t19737, t19739, t19744, t19746)
}
