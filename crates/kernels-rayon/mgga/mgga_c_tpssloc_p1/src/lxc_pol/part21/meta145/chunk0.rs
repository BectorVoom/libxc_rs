//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 948/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk948(t3449: f64, t3451: f64, t3247: f64, t461: f64, t2244: f64, t1177: f64, t1178: f64, t2250: f64, t3293: f64, t3295: f64, t3299: f64, t3302: f64, t3305: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3452 = t3449 * t3451;
    let t3455 = t461 * t3247;
    let t3456 = t3455 * t2244;
    let t3457 = t1177 * t3456;
    let t3460 = t1178 * t2250;
    let t3461 = t1177 * t3460;
    let t3464 = 5.0_f64 / 18.0_f64 * t3293;
    let t3469 = -t3464 + 2.0_f64 / 9.0_f64 * t3295 + t3299 / 18.0_f64 - t3302 / 3.0_f64 - t3305 / 6.0_f64;
    (t3452, t3455, t3456, t3457, t3460, t3461, t3464, t3469)
}
