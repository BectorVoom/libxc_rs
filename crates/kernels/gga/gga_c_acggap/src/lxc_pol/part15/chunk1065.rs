//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1065/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1065<F: Float>(t30375: F, t32456: F, t32458: F, t34349: F, t34351: F, t34364: F, t37047: F, t39209: F, t39213: F, t39217: F, t39222: F, t39226: F, t39228: F, t39230: F, t39232: F, t39236: F, t39240: F, t39243: F) -> (F,) {
    let t41538 = -0.15095084299009992993e-1 * t34349 + 0.31448092289604152069e-2 * t39209 + 0.20965394859736101379e-3 * t39213 - 0.21437009059034868486e-3 * t39217 + 0.94344276868812456207e-3 * t39222 + 0.62896184579208304138e-3 * t39226 + 0.37737710747524982483e-2 * t39228 - 0.85748036236139473944e-3 * t39230 - 0.94344276868812456204e-2 * t39232 - 0.85748036236139473944e-3 * t39236 + 0.75475421495049964965e-2 * t34351 - 0.85748036236139473944e-3 * t39240 - 0.57165357490759649296e-3 * t39243 + 0.12579236915841660828e-2 * t30375 - t37047 - t32456 + t34364 - t32458;
    (t41538,)
}
