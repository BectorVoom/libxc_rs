//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 818/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk818(t8791: f64, t9427: f64, t2385: f64, t309: f64, t2132: f64, t2131: f64, t322: f64, t2138: f64, t2147: f64, t463: f64, t1659: f64, t2146: f64, t2222: f64, t2236: f64, t2395: f64, t2400: f64, t616: f64, t7912: f64, t8349: f64, t8400: f64, t9003: f64, t9402: f64, t9407: f64, t9409: f64, t9414: f64, t9418: f64, t9422: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9428 = t9427 * t8791;
    let t9431 = t2385 * t309;
    let t9432 = t2132 * t9431;
    let t9433 = t2131 * t9432;
    let t9435 = t2385 * t322;
    let t9436 = t2132 * t9435;
    let t9437 = t2138 * t9436;
    let t9440 = t2147 * t2385 * t463;
    let t9445 = 0.4336814094102599731e0_f64 * t8400 * t9402 + 0.8673628188205199462e0_f64 * t9003 * t2236 - 0.8673628188205199462e0_f64 * t9407 + 0.8673628188205199462e0_f64 * t9409 + 0.8673628188205199462e0_f64 * t7912 * t2395 + 0.8673628188205199462e0_f64 * t2146 * t9414 + 0.8673628188205199462e0_f64 * t2146 * t9418 - 0.4336814094102599731e0_f64 * t616 * t9422 + 0.4336814094102599731e0_f64 * t7912 * t2400 + t8349 - 0.8673628188205199462e0_f64 * t8400 * t9428 - 0.8673628188205199462e0_f64 * t9433 + 0.8673628188205199462e0_f64 * t9437 + 0.8673628188205199462e0_f64 * t2146 * t9440 - 0.65854491829355115987e0_f64 * t2222 * t1659;
    (t9428, t9431, t9432, t9433, t9435, t9436, t9437, t9440, t9445)
}
