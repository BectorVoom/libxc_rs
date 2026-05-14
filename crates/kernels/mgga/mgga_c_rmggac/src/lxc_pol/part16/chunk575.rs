//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 575/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk575<F: Float>(t1973: F, t8577: F, t128: F, t1528: F, t118: F, t2001: F, t675: F, t2191: F, t2286: F, t1603: F, t1986: F, t2289: F, t1616: F, t2310: F, t1654: F, t446: F, t597: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8578 = t8577 * t1973;
    let t8580 = t128 * t1528;
    let t8581 = t118 * t8580;
    let t8582 = t2001 * t8581;
    let t8583 = t675 * t8582;
    let t8585 = t2191 * t2286;
    let t8587 = t1986 * t1603;
    let t8588 = t675 * t8587;
    let t8590 = t2191 * t2289;
    let t8592 = t1986 * t1616;
    let t8593 = t675 * t8592;
    let t8595 = t2191 * t2310;
    let t8597 = t1986 * t1654;
    let t8598 = t675 * t8597;
    let t8601 = t446 * t597;
    (t8578, t8582, t8583, t8585, t8587, t8588, t8590, t8592, t8593, t8595, t8597, t8598, t8601)
}
