//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1059/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1059<F: Float>(t14369: F, t606: F, t4401: F, t10561: F, t10563: F, t2394: F, t262: F, t10569: F, t10574: F, t10566: F, t10568: F, t11075: F, t14340: F, t14343: F, t14345: F, t14352: F, t14353: F, t14364: F, t14365: F, t1544: F, t1940: F, t198: F, t2403: F, t2404: F, t2430: F, t2832: F, t4343: F, t4546: F, t4556: F, t775: F, t9394: F) -> (F, F, F, F, F, F) {
    let t14370 = t14369 * t606;
    let t14372 = 24.0 * t4401 * t14370;
    let t14373 = 8.0 * t10561;
    let t14374 = 2.0 * t10563;
    let t14375 = t2394 * t262;
    let t14379 = 0.4883052614935078681e-3 * t10569;
    let t14380 = 0.18311447306006545054e-3 * t10574;
    let t14381 = 3.0 * t11075 * t1544 * t2403 + 6.0 * t14353 * t2403 * t775 - 6.0 * t14365 * t2403 * t4556 + 6.0 * t14375 * t1544 * t198 - t1940 * t2832 * t4556 + 6.0 * t2403 * t2404 * t4343 + 3.0 * t2403 * t2430 * t4546 + t10566 - t10568 + t14340 + t14343 + t14345 + t14352 + t14364 + t14372 + t14373 + t14374 + t14379 - t14380 + t9394;
    (t14372, t14373, t14374, t14379, t14380, t14381)
}
