//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1013/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1013(t75864: f64, t75866: f64, t75887: f64, t1356: f64, t41063: f64, t8041: f64, t41015: f64, t70086: f64, t71343: f64, t8571: f64, t71346: f64, t1981: f64, t676: f64, t708: f64, t8512: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78340 = 0.38430329123504567781e-4_f64 * t75864;
    let t78341 = 0.38430329123504567781e-4_f64 * t75866;
    let t78349 = 0.44903406381989282115e-1_f64 * t75887;
    let t78352 = 0.11974241701863808564e0_f64 * t1356 * t8041 * t41063;
    let t78355 = 0.11974241701863808564e0_f64 * t1356 * t8041 * t41015;
    let t78359 = 0.43368970657079495308e-4_f64 * t70086;
    let t78361 = t8571 * t71343;
    let t78362 = 0.12769379967989351819e-4_f64 * t78361;
    let t78363 = t8571 * t71346;
    let t78364 = 0.85129199786595678796e-5_f64 * t78363;
    let t78367 = t8512 * t1981 * t676 * t708;
    (t78340, t78341, t78349, t78352, t78355, t78359, t78362, t78364, t78367)
}
