//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1191/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1191(t18630: f64, t18673: f64, t18789: f64, t18906: f64, t300: f64, t3400: f64, t6084: f64, t4883: f64, t1164: f64, t18247: f64, t18249: f64, t18251: f64, t18257: f64, t18261: f64, t18264: f64, t18268: f64, t18270: f64, t18273: f64, t18278: f64, t18282: f64, t18285: f64, t18672: f64, t18676: f64, t18679: f64) -> (f64, f64, f64) {
    let t18909 = t300 * (t18630 + t18673 + t18789 + t18906);
    let t18910 = t3400 * t6084;
    let t18911 = t18910 * t4883;
    let t18913 = 0.17315859105681463759e2_f64 * t1164 * t18911;
    let t18914 = -t18247 - t18249 - t18251 - t18257 + t18261 + t18264 + t18268 - t18270 - t18273 - t18278 + t18282 - t18285 + t18909 - t18913 - t18672 + t18676 + t18679;
    (t18909, t18913, t18914)
}
