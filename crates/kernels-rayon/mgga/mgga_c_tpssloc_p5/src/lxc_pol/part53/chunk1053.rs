//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1053/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1053(t123373: f64, t123981: f64, t124292: f64, t124383: f64, t124428: f64, t124472: f64, t124552: f64, t124584: f64, t1858: f64, t8811: f64, t2105: f64, t7945: f64) -> (f64, f64, f64) {
    let t124587 = t123373 + t123981 + t124292 + t124383 + t124428 + t124472 + t124552 + t124584;
    let t124591 = t8811 * t1858;
    let t124596 = t7945 * t2105;
    (t124587, t124591, t124596)
}
