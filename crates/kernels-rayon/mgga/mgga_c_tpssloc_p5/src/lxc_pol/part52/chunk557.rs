//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 557/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk557(t3068: f64, t3575: f64, t1244: f64, t1230: f64, t820: f64, t1089: f64, t415: f64, t1236: f64, t225: f64, t1239: f64, t496: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3576 = t3575 * t3068;
    let t3577 = t1244 * t3576;
    let t3578 = t820 * t1230;
    let t3584 = 1.0_f64 / t415 / t1089;
    let t3593 = t1236 * t225;
    let t3597 = 1.0_f64 / t1239 / t496;
    let t3598 = t68 * t3597;
    (t3577, t3578, t3584, t3593, t3597, t3598)
}
