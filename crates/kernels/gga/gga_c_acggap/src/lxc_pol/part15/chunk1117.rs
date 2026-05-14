//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1117/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1117<F: Float>(t315: F, t323: F, t9971: F, t2230: F, t40653: F, t2146: F, t2147: F, t33274: F, t33278: F, t33281: F, t33284: F, t33286: F, t33293: F, t38092: F, t38430: F, t38432: F, t38441: F, t38443: F, t38453: F, t556: F, t8400: F, t8402: F, t9367: F) -> (F,) {
    let t42220 = t315 * t9971 * t323;
    let t42222 = t40653 * t2230;
    let t42225 = 0.8673628188205199462e0 * t33274 - 0.8673628188205199462e0 * t33278 + 0.8673628188205199462e0 * t33281 - t33284 + 0.17347256376410398924e1 * t2146 * t2147 * t9367 * t556 - 0.34694512752820797848e1 * t33286 - t38430 - 0.17347256376410398924e1 * t38432 + 0.8673628188205199462e0 * t8400 * t38092 * t8402 - t33293 + t38441 + 0.17347256376410398924e1 * t38443 - 0.65854491829355115987e0 * t42220 + 0.8673628188205199462e0 * t42222 + 0.34694512752820797848e1 * t38453;
    (t42225,)
}
