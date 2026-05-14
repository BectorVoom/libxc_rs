//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 503/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk503<F: Float>(t3383: F, t538: F, t1014: F, t542: F, t133: F, t135: F, t1013: F, t2057: F, t554: F, t1722: F, t1733: F, t2066: F, t3083: F, t3086: F, t3090: F, t3093: F, t3097: F) -> (F, F, F, F, F, F) {
    let t3384 = t3383 * t538;
    let t3387 = t542 * t1014;
    let t3392 = t133 * t135;
    let t3393 = t2057 * t1013;
    let t3394 = t3393 * t554;
    let t3404 = -0.44452000728395061731e-1 * t1722 - t2066 + 0.55565000910493827163e-2 * t1733 - 0.44452000728395061731e-1 * t3083 + 0.55565000910493827163e-2 * t3086 + 0.22226000364197530865e-1 * t3090 - 0.33339000546296296298e-1 * t3093 + 0.33339000546296296298e-1 * t3097;
    (t3384, t3387, t3392, t3393, t3394, t3404)
}
