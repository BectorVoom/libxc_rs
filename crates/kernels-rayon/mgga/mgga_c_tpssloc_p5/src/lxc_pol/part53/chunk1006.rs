//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1006/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1006(t1992: f64, t550: f64, t6976: f64, t93505: f64, t33285: f64, t6883: f64, t33284: f64, t6897: f64, t794: f64, t22897: f64, t27075: f64, t27078: f64) -> (f64, f64, f64, f64, f64) {
    let t122488 = t1992 * t6976 * t93505 * t550;
    let t122503 = t6883 * t33285;
    let t122507 = t6897 * t794 * t33284;
    let t122510 = t1992 * t22897 * t27075;
    let t122513 = t1992 * t6976 * t27078;
    (t122488, t122503, t122507, t122510, t122513)
}
