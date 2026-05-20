//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3327/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3327<F: Float>(t14353: F, t14468: F, t18850: F, t198: F, t207: F, t2403: F, t2430: F, t40067: F, t40072: F, t40167: F, t40171: F, t40184: F, t4343: F, t4546: F, t61358: F, t61387: F, t61429: F, t62269: F, t62270: F, t62273: F, t62518: F, t62545: F, t63055: F, t63088: F, t63129: F, t892: F) -> F {
    let t63145 = t198 * t207 * (t61358 + t61387 + t61429 + t62518 + t62545 + t63055 + t63088 + t63129) * t892 + t40067 - t40072 + F::new(12.0) * t2403 * t14353 * t4343 + t62269 + F::new(3.0) * t2403 * t18850 * t2430 + t40167 - t40171 - t62270 - t40184 + t62273 + F::new(6.0) * t2403 * t4546 * t14468;
    t63145
}
