//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 957/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk957(t74278: f64, t74287: f64, t74295: f64, t74316: f64, t74324: f64, t74327: f64, t74330: f64, t74281: f64, t74284: f64, t74290: f64, t74299: f64, t74302: f64, t74305: f64, t74309: f64, t74314: f64, t74319: f64, t74321: f64) -> f64 {
    let t76955 = 0.72042316457491791901e-3_f64 * t74278;
    let t76957 = 0.2553875993597870364e-4_f64 * t74287;
    let t76959 = 0.1702583995731913576e-4_f64 * t74295;
    let t76965 = 0.85129199786595678799e-5_f64 * t74316;
    let t76968 = 0.15961724959986689775e-4_f64 * t74324;
    let t76969 = 0.3192344991997337955e-4_f64 * t74327;
    let t76970 = 0.47885174879960069325e-4_f64 * t74330;
    let t76971 = -t76955 - 0.35038612185802734376e-6_f64 * t74281 - t74284 + t76957 - 0.8759653046450683594e-6_f64 * t74290 + t76959 - 0.10511583655740820313e-5_f64 * t74299 + 0.10511583655740820313e-5_f64 * t74302 - 0.10511583655740820313e-5_f64 * t74305 - 0.35038612185802734376e-6_f64 * t74309 + 0.52557918278704101564e-6_f64 * t74314 - t76965 - 0.87596530464506835935e-6_f64 * t74319 + 0.31062809106223861415e-2_f64 * t74321 - t76968 + t76969 - t76970;
    t76971
}
