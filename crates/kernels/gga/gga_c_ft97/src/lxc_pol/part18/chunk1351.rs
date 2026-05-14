//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1351/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1351<F: Float>(t105433: F, t105457: F, t105459: F, t105427: F, t105431: F, t105438: F, t105442: F, t105446: F, t105450: F, t105455: F, t95078: F, t95087: F, t105482: F, t105465: F, t105467: F, t105471: F, t105476: F, t105480: F, t105487: F, t105491: F, t105495: F, t105499: F, t95094: F, t95100: F) -> (F, F) {
    let t105981 = t105433 / 54.0;
    let t105989 = 2.0 / 27.0 * t105457;
    let t105990 = 2.0 / 27.0 * t105459;
    let t105991 = 8.0 / 9.0 * t105427 - 8.0 / 27.0 * t105431 + t105981 + t105438 / 2.0 - t95078 / 54.0 - t95087 / 81.0 + 2.0 / 9.0 * t105442 - 2.0 / 9.0 * t105446 + 2.0 / 27.0 * t105450 - t105455 / 36.0 + t105989 + t105990;
    let t105997 = 2.0 / 27.0 * t105482;
    let t106004 = -2.0 / 9.0 * t105465 + 4.0 / 81.0 * t105467 + 4.0 / 9.0 * t105471 + 2.0 / 9.0 * t105476 - 4.0 * t105480 - t105997 + t95094 / 27.0 - t105487 / 3.0 + 4.0 / 81.0 * t95100 - 2.0 * t105491 + 4.0 / 27.0 * t105495 - t105499 / 3.0;
    (t105991, t106004)
}
