//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 956/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk956<F: Float>(t119444: F, t119451: F, t119456: F, t119465: F, t119468: F, t119471: F, t125228: F, t125238: F, t125244: F, t125248: F, t125251: F, t125254: F, t125257: F, t125261: F, t125265: F, t125269: F, t125274: F, t2247: F, t32136: F, t32138: F, t32149: F, t32151: F, t32154: F, t32156: F, t33613: F, t33621: F, t33625: F, t6972: F, t8435: F) -> (F,) {
    let t125277 = -5.0 / 24.0 * t119465 * t33613 - 5.0 / 24.0 * t32136 * t125228 + 5.0 / 72.0 * t119468 * t33621 + 5.0 / 72.0 * t2247 * t8435 * t6972 * t33621 + 5.0 / 72.0 * t32149 * t125238 + 5.0 / 72.0 * t119471 * t33625 + 5.0 / 72.0 * t32154 * t125244 + 5.0 / 72.0 * t32154 * t125248 - 5.0 / 24.0 * t125251 * t32138 + 5.0 / 72.0 * t125254 * t32151 + 5.0 / 72.0 * t125257 * t32156 + 5.0 / 6.0 * t119444 * t125261 - 5.0 / 18.0 * t119451 * t125265 - 5.0 / 18.0 * t119456 * t125269 - 5.0 / 18.0 * t119451 * t125274;
    (t125277,)
}
