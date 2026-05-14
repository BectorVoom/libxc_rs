//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1273/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1273<F: Float>(t113465: F, t113462: F, t113469: F, t113472: F, t113476: F, t113481: F, t113484: F, t113487: F, t113491: F, t113495: F, t113499: F, t113503: F, t113511: F, t113516: F, t113519: F, t113522: F, t113527: F, t113530: F, t113534: F, t113536: F, t99537: F, t99545: F, t99801: F) -> (F, F) {
    let t114420 = t113465 / 12.0;
    let t114431 = t113462 / 12.0 + t114420 - t113469 / 9.0 - 2.0 / 27.0 * t113472 + t113476 / 18.0 + t113481 / 24.0 + t113484 / 3.0 - t113487 / 36.0 - t113491 / 54.0 - t113495 / 18.0 + t113499 / 9.0 + t113503 / 9.0;
    let t114444 = t113511 / 9.0 - t113516 / 27.0 - 4.0 / 9.0 * t113519 - 2.0 / 9.0 * t113522 + t113527 / 3.0 + t113530 / 3.0 + t113534 / 3.0 + 2.0 / 3.0 * t113536 + t99801 - 8.0 / 27.0 * t99537 + t99545 / 9.0;
    (t114431, t114444)
}
