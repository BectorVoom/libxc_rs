//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 818/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk818<F: Float>(t8791: F, t9427: F, t2385: F, t309: F, t2132: F, t2131: F, t322: F, t2138: F, t2147: F, t463: F, t1659: F, t2146: F, t2222: F, t2236: F, t2395: F, t2400: F, t616: F, t7912: F, t8349: F, t8400: F, t9003: F, t9402: F, t9407: F, t9409: F, t9414: F, t9418: F, t9422: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9428 = t9427 * t8791;
    let t9431 = t2385 * t309;
    let t9432 = t2132 * t9431;
    let t9433 = t2131 * t9432;
    let t9435 = t2385 * t322;
    let t9436 = t2132 * t9435;
    let t9437 = t2138 * t9436;
    let t9440 = t2147 * t2385 * t463;
    let t9445 = F::cast_from(0.4336814094102599731e0_f64) * t8400 * t9402 + F::cast_from(0.8673628188205199462e0_f64) * t9003 * t2236 - F::cast_from(0.8673628188205199462e0_f64) * t9407 + F::cast_from(0.8673628188205199462e0_f64) * t9409 + F::cast_from(0.8673628188205199462e0_f64) * t7912 * t2395 + F::cast_from(0.8673628188205199462e0_f64) * t2146 * t9414 + F::cast_from(0.8673628188205199462e0_f64) * t2146 * t9418 - F::cast_from(0.4336814094102599731e0_f64) * t616 * t9422 + F::cast_from(0.4336814094102599731e0_f64) * t7912 * t2400 + t8349 - F::cast_from(0.8673628188205199462e0_f64) * t8400 * t9428 - F::cast_from(0.8673628188205199462e0_f64) * t9433 + F::cast_from(0.8673628188205199462e0_f64) * t9437 + F::cast_from(0.8673628188205199462e0_f64) * t2146 * t9440 - F::cast_from(0.65854491829355115987e0_f64) * t2222 * t1659;
    (t9428, t9431, t9432, t9433, t9435, t9436, t9437, t9440, t9445)
}
