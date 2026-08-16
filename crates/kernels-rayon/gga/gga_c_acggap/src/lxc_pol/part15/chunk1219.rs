//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1219/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1219(t30375: f64, t32456: f64, t32458: f64, t34349: f64, t34351: f64, t34364: f64, t37047: f64, t39209: f64, t39213: f64, t39217: f64, t39222: f64, t39226: f64, t39228: f64, t39230: f64, t39232: f64, t39236: f64, t39240: f64, t39243: f64) -> f64 {
    let t41538 = -0.15095084299009992993e-1_f64 * t34349 + 0.31448092289604152069e-2_f64 * t39209 + 0.20965394859736101379e-3_f64 * t39213 - 0.21437009059034868486e-3_f64 * t39217 + 0.94344276868812456207e-3_f64 * t39222 + 0.62896184579208304138e-3_f64 * t39226 + 0.37737710747524982483e-2_f64 * t39228 - 0.85748036236139473944e-3_f64 * t39230 - 0.94344276868812456204e-2_f64 * t39232 - 0.85748036236139473944e-3_f64 * t39236 + 0.75475421495049964965e-2_f64 * t34351 - 0.85748036236139473944e-3_f64 * t39240 - 0.57165357490759649296e-3_f64 * t39243 + 0.12579236915841660828e-2_f64 * t30375 - t37047 - t32456 + t34364 - t32458;
    t41538
}
