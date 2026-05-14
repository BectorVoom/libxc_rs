//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1421/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1421<F: Float>(t22768: F, t22778: F, t26234: F, t26238: F, t26249: F, t2662: F, t30648: F, t30651: F, t30653: F, t30668: F, t30676: F, t30680: F, t30693: F, t34458: F, t9469: F, t2155: F, t34077: F, t8077: F) -> (F, F) {
    let t34460 = -0.29272321618148349056e-1 * t30648 - 0.14636160809074174528e-1 * t30651 + 0.1047928639570397803e0 * t30653 + 0.19207560116895242163e0 * t30668 - 0.1047928639570397803e0 * t30676 + 0.25426783770825854452e1 * t30680 + 0.22852785214883496467e0 * t30693 - 0.39006997830244208535e0 * t9469 * t2662 + 0.1590300183910403919e-2 * t22768 + 0.1713958891116262235e0 * t22778 - 0.48787202696913915094e-3 * t26234 + 0.27744253502182016457e1 * t26238 + 0.51418766733487867048e0 * t26249 + 0.29272321618148349055e-1 * t34458;
    let t34463 = t2155 * t8077 * t34077;
    (t34460, t34463)
}
