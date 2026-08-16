//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1169/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1169(t1751: f64, t7284: f64, t3247: f64, t497: f64, t24574: f64, t8067: f64, t477: f64, t3502: f64, t491: f64, t24813: f64, t1209: f64, t1419: f64, t6794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27426 = t7284 * t1751;
    let t27444 = t497 * t3247;
    let t27451 = t24574 * t8067;
    let t27460 = t477 * t1751;
    let t27488 = t3502 * t491;
    let t27489 = t24813 * t27488;
    let t27495 = t1209 * t491;
    let t27496 = t24813 * t27495;
    let t27505 = t1419 * t6794;
    (t27426, t27444, t27451, t27460, t27489, t27495, t27496, t27505)
}
