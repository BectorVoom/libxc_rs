//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1069/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1069(t11882: f64, t11885: f64, t11903: f64, t11906: f64, t11908: f64, t12235: f64, t12236: f64, t12237: f64, t12238: f64, t12239: f64, t12240: f64, t12243: f64, t12244: f64, t12245: f64, t12246: f64, t12247: f64, t12251: f64, t12252: f64, t12253: f64) -> f64 {
    let t12644 = -t12235 - t12236 + t12237 + t12238 + t12239 + t12240 - 0.90579542097823505428e-7_f64 * t11882 - 0.52838066223730378166e-7_f64 * t11885 + t12243 - t12244 - t12245 - t12246 - t12247 + 0.90579542097823505428e-7_f64 * t11903 - 0.18115908419564701086e-6_f64 * t11906 + 0.18115908419564701086e-6_f64 * t11908 - t12251 - t12252 + t12253;
    t12644
}
