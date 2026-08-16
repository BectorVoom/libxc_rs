//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1063/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1063(t11914: f64, t11919: f64, t11882: f64, t11885: f64, t11903: f64, t11906: f64, t11908: f64, t12235: f64, t12236: f64, t12237: f64, t12238: f64, t12239: f64, t12240: f64, t12243: f64, t12244: f64, t12245: f64, t12246: f64, t12247: f64, t12251: f64) -> f64 {
    let t12252 = 0.21102562238076876322e-7_f64 * t11914;
    let t12253 = 0.39291224566445086216e-8_f64 * t11919;
    let t12254 = -t12235 - t12236 + t12237 + t12238 + t12239 + t12240 - 0.90579542097823505425e-7_f64 * t11882 - 0.52838066223730378165e-7_f64 * t11885 + t12243 - t12244 - t12245 - t12246 - t12247 + 0.90579542097823505425e-7_f64 * t11903 - 0.18115908419564701085e-6_f64 * t11906 + 0.18115908419564701085e-6_f64 * t11908 - t12251 - t12252 + t12253;
    t12254
}
