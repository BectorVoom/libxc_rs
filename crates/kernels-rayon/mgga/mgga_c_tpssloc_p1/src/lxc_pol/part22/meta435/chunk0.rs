//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1772/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1772(t5480: f64, t9398: f64, t662: f64, t1449: f64, t2: f64, t584: f64, t2349: f64, t5484: f64, t19503: f64, t103: f64, t100: f64, t12774: f64, t12795: f64, t1447: f64, t19489: f64, t19493: f64, t19499: f64, t19504: f64, t4060: f64, t4064: f64, t5469: f64, t5472: f64, t5475: f64, t657: f64, t663: f64, t92: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19513 = t9398 * t5480;
    let t19514 = t19513 * t662;
    let t19517 = t1449 * t2;
    let t19518 = t19517 * t584;
    let t19521 = t2349 * t5484;
    let t19522 = t19521 * t662;
    let t19525 = -t19503;
    let t19526 = t103 * t19525;
    let t19529 = -50.0_f64 / 27.0_f64 * t657 * t5469 - 10.0_f64 / 27.0_f64 * t92 * t19489 + 20.0_f64 / 9.0_f64 * t12774 * t19493 - 25.0_f64 / 9.0_f64 * t657 * t5472 + 10.0_f64 / 9.0_f64 * t92 * t19499 + 5.0_f64 / 3.0_f64 * t92 * t19504 + 200.0_f64 / 27.0_f64 * t5475 * t663 - 100.0_f64 / 27.0_f64 * t1447 * t4060 + 50.0_f64 / 9.0_f64 * t1447 * t4064 - 10.0_f64 / 27.0_f64 * t100 * t19514 - 20.0_f64 / 9.0_f64 * t12795 * t19518 + 10.0_f64 / 9.0_f64 * t100 * t19522 + 5.0_f64 / 3.0_f64 * t100 * t19526;
    (t19514, t19518, t19522, t19525, t19526, t19529)
}
