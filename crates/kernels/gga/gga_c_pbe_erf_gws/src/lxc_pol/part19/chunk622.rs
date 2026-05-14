//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 622/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk622<F: Float>(t251: F, t3583: F, t1619: F, t1675: F, t2536: F, t256: F, t2651: F, t2655: F, t2657: F, t3388: F, t3389: F, t3394: F, t3401: F, t3405: F, t1780: F, t3409: F, t3413: F, t3417: F, t3419: F, t3447: F, t3449: F, t3453: F, t3458: F, t3481: F, t3490: F, t3495: F, t3496: F) -> (F, F, F) {
    let t3584 = t3583 * t251;
    let t3591 = t1619 + t3584 * t256 / 3.0 - 4.0 / 45.0 * t2536 + t3388 + t3389 + t3394 + 2.0 / 3.0 * t2651 + 0.12155555555555555555e0 * t2655 + 4.0 / 9.0 * t2657 - t1675 + t3401 + t3405;
    let t3592 = -t3409 + t3413 - t3417 - t3419 - t3447 + t3449 + t3453 + t3458 - t1780 + t3481 + t3490 + t3495 + t3496;
    (t3584, t3591, t3592)
}
