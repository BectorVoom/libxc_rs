//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 975/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk975(t11330: f64, t11305: f64, t11308: f64, t11310: f64, t11311: f64, t11312: f64, t11314: f64, t11316: f64, t11318: f64, t11320: f64, t11323: f64, t11324: f64, t11325: f64, t11328: f64, t8168: f64, t8177: f64, t8184: f64, t8188: f64) -> (f64, f64) {
    let t11331 = 0.0005493466511025948_f64 * t11330;
    let t11332 = -0.4740006021527056_f64 * t11305 + t11308 - t11310 - t8168 - t8177 - t11311 - t11312 - t11314 - t11316 + t11318 + t11320 + t11323 + t8184 - t8188 - t11324 - 1.825614615114074_f64 * t11325 - t11328 - t11331;
    (t11331, t11332)
}
