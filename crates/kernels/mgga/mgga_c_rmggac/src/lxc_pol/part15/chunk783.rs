//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 783/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk783<F: Float>(t8563: F, t8565: F, t8569: F, t8572: F, t8578: F, t8583: F, t8585: F, t8588: F, t8590: F, t8593: F, t8595: F, t8598: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t38254 = F::cast_from(0.27274661654245341728e-1_f64) * t8563;
    let t38255 = F::cast_from(0.68186654135613354322e-2_f64) * t8565;
    let t38256 = F::cast_from(0.68186654135613354322e-2_f64) * t8569;
    let t38257 = F::cast_from(0.85129199786595678796e-5_f64) * t8572;
    let t38260 = F::cast_from(0.85129199786595678796e-5_f64) * t8578;
    let t38261 = F::cast_from(0.85129199786595678796e-5_f64) * t8583;
    let t38262 = F::cast_from(0.25538759935978703638e-4_f64) * t8585;
    let t38263 = F::cast_from(0.25538759935978703638e-4_f64) * t8588;
    let t38266 = F::cast_from(0.25538759935978703638e-4_f64) * t8590;
    let t38267 = F::cast_from(0.25538759935978703638e-4_f64) * t8593;
    let t38268 = F::cast_from(0.85129199786595678796e-5_f64) * t8595;
    let t38269 = F::cast_from(0.85129199786595678796e-5_f64) * t8598;
    (t38254, t38255, t38256, t38257, t38260, t38261, t38262, t38263, t38266, t38267, t38268, t38269)
}
