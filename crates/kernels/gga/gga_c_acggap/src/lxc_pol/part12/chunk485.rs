//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 485/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk485<F: Float>(t2327: F, t598: F, t545: F, t615: F, t2179: F, t2180: F, t2182: F, t2183: F, t2184: F, t2185: F, t2189: F, t2190: F, t2191: F, t2258: F, t2261: F, t2265: F, t2269: F, t2271: F, t2275: F, t2279: F, t2283: F, t2285: F) -> (F, F, F) {
    let t2328 = t598 * t2327;
    let t2338 = t615 * t545;
    let t2372 = t2179 - t2180 + t2182 - t2183 - t2184 - t2185 - F::new(0.34299214494455789578e-2) * t2258 - t2189 + t2190 + t2261 / F::new(48.0) - F::new(0.21437009059034868486e-3) * t2265 + F::new(0.31448092289604152069e-3) * t2269 + t2191 - t2271 / F::new(48.0) - t2275 / F::new(64.0) - t2279 / F::new(192.0) - F::new(0.7640625e-2) * t2283 - t2285 / F::new(24.0);
    (t2328, t2338, t2372)
}
