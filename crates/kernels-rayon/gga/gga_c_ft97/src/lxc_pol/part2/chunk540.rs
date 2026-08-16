//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 540/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk540(t140: f64, t1722: f64, t1733: f64, t2066: f64, t3083: f64, t3086: f64, t3090: f64, t3093: f64, t3097: f64, t550: f64, t133: f64, t1010: f64, t1015: f64, t2001: f64, t3348: f64, t3350: f64, t3356: f64, t3381: f64, t3384: f64, t3387: f64, t3392: f64, t3394: f64, t399: f64) -> (f64, f64, f64, f64) {
    let t141 = 0.1e-59_f64 < t140;
    let t3404 = -0.44452000728395061731e-1_f64 * t1722 - t2066 + 0.55565000910493827163e-2_f64 * t1733 - 0.44452000728395061731e-1_f64 * t3083 + 0.55565000910493827163e-2_f64 * t3086 + 0.22226000364197530865e-1_f64 * t3090 - 0.33339000546296296298e-1_f64 * t3093 + 0.33339000546296296298e-1_f64 * t3097;
    let t3405 = t550 * t3404;
    let t3406 = t133 * t3405;
    let t3408 = piecewise3(t141, 2.0_f64 * t3348 - 0.1208182677680765956e1_f64 * t3350 * t399 + 0.1208182677680765956e1_f64 * t1010 * t399 - 2.0_f64 * t2001 * t3356 + 2.0_f64 * t3381 - 2.0_f64 * t2001 * t3384 + 0.60409133884038297798e0_f64 * t3387 * t399 - 0.60409133884038297798e0_f64 * t1015 * t399 + 2.0_f64 * t3392 * t3394 - t3406, 0.0_f64);
    (t3404, t3405, t3406, t3408)
}
