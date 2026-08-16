//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1013/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1013<F: Float>(t75864: F, t75866: F, t75887: F, t1356: F, t41063: F, t8041: F, t41015: F, t70086: F, t71343: F, t8571: F, t71346: F, t1981: F, t676: F, t708: F, t8512: F) -> (F, F, F, F, F, F, F, F, F) {
    let t78340 = F::cast_from(0.38430329123504567781e-4_f64) * t75864;
    let t78341 = F::cast_from(0.38430329123504567781e-4_f64) * t75866;
    let t78349 = F::cast_from(0.44903406381989282115e-1_f64) * t75887;
    let t78352 = F::cast_from(0.11974241701863808564e0_f64) * t1356 * t8041 * t41063;
    let t78355 = F::cast_from(0.11974241701863808564e0_f64) * t1356 * t8041 * t41015;
    let t78359 = F::cast_from(0.43368970657079495308e-4_f64) * t70086;
    let t78361 = t8571 * t71343;
    let t78362 = F::cast_from(0.12769379967989351819e-4_f64) * t78361;
    let t78363 = t8571 * t71346;
    let t78364 = F::cast_from(0.85129199786595678796e-5_f64) * t78363;
    let t78367 = t8512 * t1981 * t676 * t708;
    (t78340, t78341, t78349, t78352, t78355, t78359, t78362, t78364, t78367)
}
