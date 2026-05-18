//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 635/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk635<F: Float>(t3451: F, t387: F, t1187: F, t1184: F, t1196: F, t1200: F, t3316: F, t359: F, t376: F, t1170: F, t3225: F, t373: F) -> (F, F, F, F, F, F, F) {
    let t3452 = t387 * t3451;
    let t3453 = t1187 * t3452;
    let t3455 = t1184 * t1196;
    let t3457 = t1184 * t1200;
    let t3459 = t359 * t3316;
    let t3460 = t376 * t3459;
    let t3461 = t1170 * t3460;
    let t3463 = t373 * t3225;
    (t3452, t3453, t3455, t3457, t3460, t3461, t3463)
}
