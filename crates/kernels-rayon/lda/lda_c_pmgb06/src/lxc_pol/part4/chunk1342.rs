//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1342/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1342(t13672: f64, t17466: f64, t5069: f64, t12546: f64, t17070: f64, t17593: f64, t17597: f64, t17601: f64, t17604: f64, t17607: f64, t17610: f64, t17614: f64, t17616: f64, t17620: f64, t17624: f64, t17627: f64, t17631: f64, t17634: f64) -> (f64, f64, f64) {
    let t17637 = 16.0_f64 / 45.0_f64 * t13672 * t5069 * t17466;
    let t17640 = 16.0_f64 / 15.0_f64 * t13672 * t12546 * t17070;
    let t17641 = t17593 - t17597 + t17601 + t17604 - t17607 + t17610 + t17614 + t17616 + t17620 - t17624 + t17627 - t17631 + t17634 - t17637 - t17640;
    (t17637, t17640, t17641)
}
