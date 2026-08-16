//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2095/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2095(t35577: f64, t1454: f64, t2585: f64, t2281: f64, t4044: f64, t4068: f64, t92: f64, t9384: f64, t100: f64, t9398: f64, t1406: f64, t9238: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45496 = 1.0_f64 / t35577;
    let t45656 = t2585 * t1454;
    let t45658 = t2281 * t4044;
    let t45659 = 22.0_f64 / 3.0_f64 * t45658;
    let t45688 = t2281 * t4068;
    let t45689 = 11.0_f64 / 3.0_f64 * t45688;
    let t45697 = t92 * t9384;
    let t45707 = t100 * t9398;
    let t45844 = t1406 * t9238;
    (t45496, t45656, t45659, t45689, t45697, t45707, t45844)
}
