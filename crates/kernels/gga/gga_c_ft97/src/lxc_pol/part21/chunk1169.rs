//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1169/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1169<F: Float>(t1317: F, t29707: F, t376: F, t29678: F, t89: F, t101651: F, t4505: F, t452: F, t473: F, t5675: F, t1871: F, t25893: F, t432: F, t57561: F, t29671: F, t93506: F) -> (F, F, F, F, F, F, F, F) {
    let t116641 = t1317 * t376 * t29707;
    let t116642 = 2.0 / 3.0 * t116641;
    let t116645 = t89 * t376 * t29678;
    let t116646 = 2.0 / 3.0 * t116645;
    let t116650 = t101651 * t452 * t5675 * t4505 * t473;
    let t116655 = t25893 * t1871 * t5675 * t4505 * t432;
    let t116659 = t25893 * t452 * t5675 * t57561;
    let t116661 = t93506 * t29671;
    (t116641, t116642, t116645, t116646, t116650, t116655, t116659, t116661)
}
