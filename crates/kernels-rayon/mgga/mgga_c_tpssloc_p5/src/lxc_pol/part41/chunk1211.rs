//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1211/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1211(t19231: f64, t19261: f64, t1256: f64, t18247: f64, t18249: f64, t18251: f64, t18257: f64, t18261: f64, t18264: f64, t18268: f64, t18270: f64, t18273: f64, t18278: f64, t18282: f64, t18285: f64, t18672: f64, t18676: f64, t18679: f64, t18909: f64, t18913: f64, t193: f64, t336: f64, t4700: f64, t5091: f64, t5095: f64) -> f64 {
    let t19262 = t19231 + t19261;
    let t19266 = t1256 * t19262 * t193 * t336 - 2.0_f64 * t4700 * t5091 * t5095 - t18247 - t18249 - t18251 - t18257 + t18261 + t18264 + t18268 - t18270 - t18273 - t18278 + t18282 - t18285 - t18672 + t18676 + t18679 + t18909 - t18913;
    t19266
}
