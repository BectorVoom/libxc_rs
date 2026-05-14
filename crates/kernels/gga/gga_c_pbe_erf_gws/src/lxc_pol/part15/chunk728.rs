//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 728/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk728<F: Float>(t1644: F, t5480: F, t639: F, t213: F, t9: F, t1623: F, t1620: F, t1673: F, t579: F, t266: F, t331: F, t265: F, t1640: F, t649: F, t1661: F, t597: F) -> (F, F, F, F, F, F, F) {
    let t5481 = t5480 * t1644;
    let t5482 = t639 * t5481;
    let t5493 = t9 * t213;
    let t5494 = t5493 * t1623;
    let t5495 = t1620 * t5494;
    let t5513 = t579 * t1673;
    let t5519 = t266 * t331;
    let t5521 = 8.0 / 405.0 * t265 * t5519;
    let t5522 = t1640 * t649;
    let t5543 = t1661 * t597;
    (t5482, t5493, t5495, t5513, t5521, t5522, t5543)
}
