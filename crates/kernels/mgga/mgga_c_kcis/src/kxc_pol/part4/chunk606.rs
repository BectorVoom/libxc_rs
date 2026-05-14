//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 606/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk606<F: Float>(t2815: F, t3438: F, t3437: F, t3226: F, t381: F, t3228: F, t388: F, t387: F, t3190: F, t358: F, t382: F, t3316: F, t1187: F, t1184: F, t1196: F, t1200: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3439 = t3438 * t2815;
    let t3440 = t3437 * t3439;
    let t3442 = t3226 * t381;
    let t3443 = t388 * t3228;
    let t3444 = t387 * t3443;
    let t3445 = t3442 * t3444;
    let t3447 = t358 * t3190;
    let t3448 = t387 * t3447;
    let t3449 = t382 * t3448;
    let t3451 = t388 * t3316;
    let t3452 = t387 * t3451;
    let t3453 = t1187 * t3452;
    let t3455 = t1184 * t1196;
    let t3457 = t1184 * t1200;
    (t3439, t3440, t3444, t3445, t3448, t3449, t3452, t3453, t3455, t3457)
}
