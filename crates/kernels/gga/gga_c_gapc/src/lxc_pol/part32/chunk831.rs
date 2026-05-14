//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 831/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk831<F: Float>(t1030: F, t11356: F, t9262: F, t144: F, t8448: F, t1971: F, t9272: F, t1734: F, t5056: F, t1743: F, t5703: F, t3709: F, t11304: F, t11306: F, t11309: F, t11314: F, t11318: F, t11323: F, t11327: F, t11330: F, t11334: F, t11337: F, t11339: F, t11345: F, t11348: F, t11351: F, t11353: F) -> (F, F, F, F, F) {
    let t11357 = t1030 * t11356;
    let t11358 = t11357 * t9262;
    let t11360 = t8448 * t144;
    let t11361 = t1971 * t11360;
    let t11362 = t1030 * t11361;
    let t11363 = t11362 * t9272;
    let t11365 = t1734 * t5056;
    let t11367 = t1743 * t11365 * t5703;
    let t11369 = t3709 * t9262;
    let t11371 = 0.90579542097823505428e-7 * t11304 - 0.90579542097823505428e-7 * t11306 + 0.33148893438893365995e-7 * t11309 + 0.17376185052903442709e-3 * t11314 + 0.17376185052903442709e-3 * t11318 - 0.25745714186718600948e-5 * t11323 - 0.35172068325509175607e-8 * t11327 + 0.33148893438893365995e-7 * t11330 - 0.12670134934408760309e-3 * t11334 - 0.12650960286458333334e-5 * t11337 - 0.20241536458333333334e-4 * t11339 + 0.12229261610243055556e-4 * t11345 - 0.17376185052903442709e-3 * t11348 - 0.20241536458333333334e-4 * t11351 + 0.54106179813099907243e-4 * t11353 - 0.21103240995305505364e-7 * t11358 + 0.39292488356234936494e-8 * t11363 - 0.26419033111865189083e-7 * t11367 - 0.6629778687778673199e-7 * t11369;
    (t11357, t11361, t11362, t11365, t11371)
}
