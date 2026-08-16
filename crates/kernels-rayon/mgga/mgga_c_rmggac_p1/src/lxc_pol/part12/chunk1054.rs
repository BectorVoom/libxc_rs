//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1054/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1054(t1364: f64, t1550: f64, t1632: f64, t1635: f64, t2124: f64, t2604: f64, t26287: f64, t26291: f64, t30204: f64, t36680: f64, t36689: f64, t41460: f64, t41551: f64, t41554: f64, t41846: f64, t41848: f64, t41850: f64, t41863: f64, t41865: f64, t4601: f64, t5204: f64, t5207: f64, t5898: f64, t665: f64, t7567: f64, t8374: f64, t8384: f64, t884: f64, t903: f64) -> f64 {
    let t41881 = -0.23948483403727617128e0_f64 * t36680 - 0.53205749866622299248e-5_f64 * t41846 - 0.2993560425465952141e-1_f64 * t41848 + t36689 - 0.5987120850931904282e-1_f64 * t41850 - 0.23948483403727617128e0_f64 * t884 * t7567 * t5898 + 0.35922725105591425692e0_f64 * t903 * t2124 * t1632 - 0.47896966807455234256e0_f64 * t1364 * t2124 * t1635 + 0.23948483403727617128e0_f64 * t2604 * t8384 - 0.42564599893297839398e-5_f64 * t41863 - 0.85129199786595678796e-5_f64 * t41865 + 0.35922725105591425692e0_f64 * t4601 * t8374 - 0.11974241701863808564e0_f64 * t1550 * t665 * t5207 - 0.23948483403727617128e0_f64 * t1550 * t665 * t5204 + 0.71845450211182851384e0_f64 * t26287 * t41551 + 0.47896966807455234256e0_f64 * t30204 * t41554 - 0.71845450211182851384e0_f64 * t26291 * t41460;
    t41881
}
