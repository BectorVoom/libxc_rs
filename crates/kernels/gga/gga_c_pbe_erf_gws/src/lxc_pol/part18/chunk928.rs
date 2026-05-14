//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 928/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk928<F: Float>(t3793: F, t8928: F, t2206: F, t3867: F, t2289: F, t3827: F, t3857: F, t3802: F, t4394: F, t2105: F, t820: F, t9482: F, t2271: F, t3861: F, t905: F, t11339: F, t823: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11492 = t8928 * t3793 / 96.0;
    let t11493 = t2206 * t3867;
    let t11494 = 7.0 / 144.0 * t11493;
    let t11495 = t2289 * t3827;
    let t11497 = t2289 * t3857;
    let t11499 = t3802 * t4394;
    let t11500 = t2105 * t820;
    let t11501 = t11499 * t11500;
    let t11502 = t9482 * t11501;
    let t11505 = t3861 * t2271;
    let t11506 = t905 * t11505;
    let t11509 = t11339 * t823;
    (t11492, t11494, t11495, t11497, t11499, t11501, t11502, t11505, t11506, t11509)
}
