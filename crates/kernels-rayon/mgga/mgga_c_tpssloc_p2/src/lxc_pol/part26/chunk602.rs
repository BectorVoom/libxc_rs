//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 602/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk602(t1184: f64, t3448: f64, t1089: f64, t460: f64, t607: f64, t3247: f64, t461: f64, t2244: f64, t1177: f64, t1178: f64, t2250: f64, t3293: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3449 = t3448 * t1184;
    let t3450 = t460 * t1089;
    let t3451 = t3450 * t607;
    let t3452 = t3449 * t3451;
    let t3455 = t461 * t3247;
    let t3456 = t3455 * t2244;
    let t3457 = t1177 * t3456;
    let t3460 = t1178 * t2250;
    let t3461 = t1177 * t3460;
    let t3464 = 5.0_f64 / 18.0_f64 * t3293;
    (t3449, t3450, t3451, t3452, t3456, t3457, t3460, t3461, t3464)
}
