//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1441/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1441(t31776: f64, t96797: f64, t1983: f64, t33136: f64, t7217: f64, t33623: f64, t6876: f64, t33214: f64, t7057: f64, t25985: f64, t8607: f64, t27171: f64, t8526: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t122587 = 2.0_f64 * t96797 * t31776;
    let t122589 = t1983 * t7217 * t33136;
    let t122590 = t6876 * t33623;
    let t122593 = 2.0_f64 * t33214 * t7057;
    let t122595 = 3.0_f64 * t8607 * t25985;
    let t122597 = t8526 * t27171;
    (t122587, t122589, t122590, t122593, t122595, t122597)
}
