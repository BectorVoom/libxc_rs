//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1051/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1051(t39285: f64, t39295: f64, t39308: f64, t39282: f64, t39289: f64, t39293: f64, t39297: f64, t39301: f64, t39306: f64, t39310: f64, t39312: f64, t39314: f64, t39316: f64, t39323: f64, t39325: f64, t4985: f64, t6473: f64, t8045: f64, t8278: f64) -> f64 {
    let t42906 = 0.39726959900411316772e-4_f64 * t39285;
    let t42909 = 0.39726959900411316772e-4_f64 * t39295;
    let t42913 = 0.66211599834018861287e-4_f64 * t39308;
    let t42924 = -0.212822999466489197e-4_f64 * t39282 + t42906 - 0.79453919800822633545e-4_f64 * t39289 - 0.5107751987195740728e-4_f64 * t39293 + t42909 - 0.85129199786595678799e-5_f64 * t39297 + 0.23942587439980034662e-4_f64 * t39301 + 0.1064114997332445985e-4_f64 * t39306 - t42913 - 0.85129199786595678799e-5_f64 * t39310 + 0.212822999466489197e-4_f64 * t39312 - 0.638468998399467591e-4_f64 * t39314 + 0.638468998399467591e-4_f64 * t39316 - 0.23948483403727617128e0_f64 * t6473 * t8045 + 0.59871208509319042821e-1_f64 * t4985 * t8278 + 0.1702583995731913576e-4_f64 * t39323 - 0.1702583995731913576e-4_f64 * t39325;
    t42924
}
