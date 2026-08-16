//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 795/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk795(t1044: f64, t248: f64, t4347: f64, t1009: f64, t1603: f64, t1011: f64, t1019: f64, t1040: f64, t1611: f64, t4353: f64, t4356: f64, t4358: f64, t4361: f64, t4398: f64, t4402: f64, t4480: f64, t4482: f64, t4485: f64, t4487: f64, t4491: f64, t4495: f64, t4500: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4636 = t248 * t1044 * t4347;
    let t4639 = t1603 * t1009;
    let t4640 = t4639 * t1011;
    let t4641 = t4640 * t1019;
    let t4644 = t1611 * t1040;
    let t4649 = -t4353 + t4356 + t4358 - t4361 + t4398 + t4402 + t4480 + t4482 - t4485 - t4487 + t4491 - t4495 - t4500;
    (t4636, t4639, t4640, t4641, t4644, t4649)
}
