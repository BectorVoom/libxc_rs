//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1277/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1277<F: Float>(t1349: F, t26514: F, t376: F, t2180: F, t6718: F, t9439: F, t2179: F, t27191: F, t609: F, t23413: F, t27416: F, t27417: F, t27423: F, t27429: F, t28: F, t3408: F, t5772: F, t5778: F, t614: F, t94230: F, t94258: F, t94260: F, t94263: F, t94265: F, t94267: F) -> (F, F, F) {
    let t104379 = t1349 * t376 * t26514 / 9.0;
    let t104381 = t9439 * t6718 * t2180;
    let t104388 = t2179 * t27191 * t609;
    let t104405 = -t104379 - 12.0 * t104381 - t94258 / 9.0 + t94260 / 81.0 + t94263 / 27.0 + t94265 / 54.0 + 8.0 * t104388 + t94267 / 27.0 + 2.0 / 9.0 * t23413 * t27417 + 2.0 / 9.0 * t23413 * t27423 - 2.0 / 27.0 * t23413 * t27429 + 2.0 / 9.0 * t5772 * t94230 * t27416 - 2.0 / 3.0 * t1349 * t28 * t5778 * t614 * t3408;
    (t104381, t104388, t104405)
}
