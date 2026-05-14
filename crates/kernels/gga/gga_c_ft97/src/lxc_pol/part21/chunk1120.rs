//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1120/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1120<F: Float>(t100496: F, t100645: F, t101512: F, t115312: F, t115316: F, t115320: F, t115324: F, t115333: F, t115337: F, t115341: F, t115349: F, t115353: F, t115362: F, t22522: F, t22534: F, t22613: F, t22619: F, t22777: F, t22796: F, t22797: F, t22798: F, t29473: F, t29474: F, t5538: F, t5569: F, t5570: F, t58268: F, t58580: F, t72: F, t73: F, t925: F, t92533: F, t92538: F, t93164: F) -> (F,) {
    let t115369 = 0.44540303667943584666e-3 * t22613 * t73 * t115312 - 0.44540303667943584666e-3 * t22619 * t73 * t115316 - 0.89080607335887169332e-3 * t22619 * t73 * t115320 - 0.89080607335887169332e-3 * t22619 * t73 * t115324 + 0.20715606998445758511e-4 * t101512 * t93164 * t72 * t58580 + 0.13362091100383075399e-2 * t92538 * t73 * t115333 + 0.53448364401532301599e-4 * t5569 * t73 * t115337 - 0.89080607335887169332e-4 * t5569 * t73 * t115341 - 0.10560293360415908094e-5 * t22796 * t22797 * t29473 * t22798 + 0.89019191601965515283e-5 * t22534 * t73 * t115349 + 0.29673063867321838428e-4 * t92533 * t73 * t115353 - 0.85124811172839506172e-2 * t100496 + 0.27568129967481981594e-4 * t5538 * t22777 * t29474 + 0.36061544906567819424e-6 * t58268 * t115362 + 0.25537443351851851852e-1 * t22522 * t5570 * t100645 * t925;
    (t115369,)
}
