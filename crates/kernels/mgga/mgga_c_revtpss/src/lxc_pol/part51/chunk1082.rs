//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1082/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1082<F: Float>(t13272: F, t32135: F, t1470: F, t644: F, t8442: F, t6972: F, t640: F, t119457: F, t36: F, t606: F, t7714: F, t119444: F, t119451: F, t119456: F, t119465: F, t119468: F, t119471: F, t125228: F, t125238: F, t125244: F, t125248: F, t125251: F, t125254: F, t2247: F, t32136: F, t32138: F, t32149: F, t32151: F, t32154: F, t32156: F, t33613: F, t33621: F, t33625: F, t8435: F) -> F {
    let t125257 = t13272 * t32135;
    let t125260 = t1470 * t644;
    let t125261 = t8442 * t125260;
    let t125265 = t8442 * t1470 * t6972;
    let t125268 = t1470 * t640;
    let t125269 = t119457 * t125268;
    let t125274 = t8442 * t7714 * t36 * t606;
    let t125277 = -F::new(5.0) / F::new(24.0) * t119465 * t33613 - F::new(5.0) / F::new(24.0) * t32136 * t125228 + F::new(5.0) / F::new(72.0) * t119468 * t33621 + F::new(5.0) / F::new(72.0) * t2247 * t8435 * t6972 * t33621 + F::new(5.0) / F::new(72.0) * t32149 * t125238 + F::new(5.0) / F::new(72.0) * t119471 * t33625 + F::new(5.0) / F::new(72.0) * t32154 * t125244 + F::new(5.0) / F::new(72.0) * t32154 * t125248 - F::new(5.0) / F::new(24.0) * t125251 * t32138 + F::new(5.0) / F::new(72.0) * t125254 * t32151 + F::new(5.0) / F::new(72.0) * t125257 * t32156 + F::new(5.0) / F::new(6.0) * t119444 * t125261 - F::new(5.0) / F::new(18.0) * t119451 * t125265 - F::new(5.0) / F::new(18.0) * t119456 * t125269 - F::new(5.0) / F::new(18.0) * t119451 * t125274;
    t125277
}
