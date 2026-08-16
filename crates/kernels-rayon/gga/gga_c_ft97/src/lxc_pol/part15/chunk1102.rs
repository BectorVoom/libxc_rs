//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1102/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1102(t147: f64, t87840: f64, t88051: f64, t21645: f64, t3699: f64, t4917: f64, t5064: f64, t1168: f64, t21181: f64, t1091: f64, t14080: f64, t1901: f64, t2599: f64, t42416: f64, t51453: f64, t65437: f64, t65508: f64, t79138: f64, t79157: f64, t79179: f64, t79182: f64, t79218: f64, t80212: f64, t81413: f64) -> (f64, f64, f64, f64, f64) {
    let t148 = 10000000.0_f64 <= t147;
    let t88053 = piecewise3(t148, 0.0_f64, t87840 + t88051);
    let t88068 = t3699 * t21645;
    let t88079 = t4917 * t5064;
    let t88098 = t21181 * t1168;
    let t88103 = -16.0_f64 / 81.0_f64 * t65437 + 8.0_f64 / 9.0_f64 * t79138 + 8.0_f64 / 9.0_f64 * t79157 + 16.0_f64 / 9.0_f64 * t65508 + 112.0_f64 / 81.0_f64 * t51453 + 4.0_f64 / 3.0_f64 * t79179 + 4.0_f64 / 3.0_f64 * t79182 + 4.0_f64 / 3.0_f64 * t79218 + 4.0_f64 / 9.0_f64 * t1901 * t2599 * t81413 * t1091 + 4.0_f64 / 9.0_f64 * t80212 + 40.0_f64 / 81.0_f64 * t1901 * t14080 * t42416 * t88098;
    (t88053, t88068, t88079, t88098, t88103)
}
