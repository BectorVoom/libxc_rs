//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1274/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1274<F: Float>(t113564: F, t113566: F, t113568: F, t113543: F, t113546: F, t113549: F, t113553: F, t113556: F, t113559: F, t113562: F, t113571: F, t99555: F, t113594: F, t113601: F, t113609: F, t113577: F, t113579: F, t113584: F, t113588: F, t113591: F, t113598: F, t113606: F, t113613: F, t99557: F) -> (F, F) {
    let t114452 = 2.0 / 27.0 * t113564;
    let t114453 = 2.0 / 27.0 * t113566;
    let t114454 = 2.0 / 81.0 * t113568;
    let t114457 = 2.0 / 3.0 * t113543 - 2.0 / 9.0 * t113546 + 2.0 / 27.0 * t113549 - t113553 / 18.0 + 4.0 / 9.0 * t113556 + 4.0 / 9.0 * t113559 - 4.0 / 27.0 * t113562 + t114452 + t114453 - t114454 - 2.0 / 9.0 * t113571 - 2.0 / 9.0 * t99555;
    let t114465 = 4.0 / 9.0 * t113594;
    let t114467 = 4.0 / 9.0 * t113601;
    let t114469 = 4.0 / 9.0 * t113609;
    let t114471 = 2.0 / 3.0 * t113577 + 2.0 / 81.0 * t113579 + 4.0 / 3.0 * t113584 + 2.0 / 3.0 * t113588 + t99557 / 54.0 + 8.0 / 27.0 * t113591 - t114465 - 2.0 / 9.0 * t113598 - t114467 - t113606 / 6.0 - t114469 - 2.0 * t113613;
    (t114457, t114471)
}
