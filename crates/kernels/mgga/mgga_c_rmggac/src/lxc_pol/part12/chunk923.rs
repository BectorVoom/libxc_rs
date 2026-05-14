//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 923/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk923<F: Float>(t118: F, t128: F, t1986: F, t1994: F, t5735: F, t30137: F, t681: F, t2034: F, t30174: F, t2310: F, t7944: F, t2191: F, t8597: F, t1364: F, t1550: F, t1632: F, t1635: F, t2124: F, t2604: F, t26287: F, t26291: F, t30204: F, t36680: F, t36689: F, t41460: F, t41551: F, t41554: F, t4601: F, t5204: F, t5207: F, t5898: F, t665: F, t7567: F, t8374: F, t8384: F, t884: F, t903: F) -> (F,) {
    let t41846 = t1994 * t1986 * t118 * t128 * t5735;
    let t41848 = t30137 * t681;
    let t41850 = t30174 * t2034;
    let t41863 = t7944 * t2310;
    let t41865 = t2191 * t8597;
    let t41881 = -0.23948483403727617128e0 * t36680 - 0.53205749866622299248e-5 * t41846 - 0.2993560425465952141e-1 * t41848 + t36689 - 0.5987120850931904282e-1 * t41850 - 0.23948483403727617128e0 * t884 * t7567 * t5898 + 0.35922725105591425692e0 * t903 * t2124 * t1632 - 0.47896966807455234256e0 * t1364 * t2124 * t1635 + 0.23948483403727617128e0 * t2604 * t8384 - 0.42564599893297839398e-5 * t41863 - 0.85129199786595678796e-5 * t41865 + 0.35922725105591425692e0 * t4601 * t8374 - 0.11974241701863808564e0 * t1550 * t665 * t5207 - 0.23948483403727617128e0 * t1550 * t665 * t5204 + 0.71845450211182851384e0 * t26287 * t41551 + 0.47896966807455234256e0 * t30204 * t41554 - 0.71845450211182851384e0 * t26291 * t41460;
    (t41881,)
}
