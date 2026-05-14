//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1190/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1190<F: Float>(t100482: F, t101588: F, t101596: F, t101598: F, t116546: F, t116549: F, t116552: F, t116555: F, t116560: F, t116564: F, t93424: F, t101616: F, t101638: F, t101662: F, t101688: F, t116569: F, t116574: F, t116579: F, t116583: F, t116587: F, t93453: F, t93458: F, t93474: F) -> (F, F) {
    let t117145 = -t100482 - 2.0 / 27.0 * t93424 + 2.0 / 3.0 * t116546 + 2.0 / 3.0 * t116549 - 4.0 / 9.0 * t116552 - 8.0 / 9.0 * t116555 + t116560 / 3.0 + t116564 / 3.0 - t101588 - t101596 + 2.0 / 27.0 * t101598;
    let t117154 = t116569 / 9.0 + t116574 / 9.0 - t116579 / 27.0 - 2.0 * t116583 + 2.0 / 3.0 * t116587 + t101616 - 8.0 / 27.0 * t101638 + t101662 + t93453 - 4.0 / 27.0 * t93458 + 8.0 / 27.0 * t93474 + t101688;
    (t117145, t117154)
}
