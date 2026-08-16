//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2148/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2148(t10817: f64, t5695: f64, t2787: f64, t5727: f64, t10296: f64, t10556: f64, t10675: f64, t10676: f64, t13551: f64, t13552: f64, t13563: f64, t13567: f64, t17173: f64, t17180: f64, t17185: f64) -> (f64, f64, f64) {
    let t17377 = 2.0_f64 * t10817 * t5695;
    let t17379 = 1.0_f64 * t2787 * t5727;
    let t17398 = 0.11958666666666666667e1_f64 * t17173 - t13551 + 0.36514074074074074073e-1_f64 * t13552 + 0.13287407407407407407e0_f64 * t13563 - t13567 - 0.91285185185185185187e-1_f64 * t10296 - t10675 - t10676 - 0.19931111111111111111e0_f64 * t17180 + 0.59793333333333333334e0_f64 * t17185 - 0.13287407407407407408e0_f64 * t10556;
    (t17377, t17379, t17398)
}
