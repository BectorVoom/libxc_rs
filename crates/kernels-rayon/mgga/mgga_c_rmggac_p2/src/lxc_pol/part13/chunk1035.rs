//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1035/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1035(t38382: f64, t38414: f64, t38363: f64, t38365: f64, t38367: f64, t38371: f64, t38375: f64, t38377: f64, t38387: f64, t38389: f64, t38391: f64, t38393: f64, t38395: f64, t38398: f64, t38404: f64, t38412: f64, t38420: f64) -> f64 {
    let t42600 = 0.2927036860455597649e0_f64 * t38382;
    let t42609 = 0.39726959900411316772e-4_f64 * t38414;
    let t42611 = 0.5107751987195740728e-4_f64 * t38363 - 0.85129199786595678799e-5_f64 * t38365 - 0.1702583995731913576e-4_f64 * t38367 - 0.212822999466489197e-4_f64 * t38371 - 0.212822999466489197e-4_f64 * t38375 - 0.1064114997332445985e-4_f64 * t38377 + t42600 + 0.1702583995731913576e-4_f64 * t38387 + 0.1702583995731913576e-4_f64 * t38389 - 0.5107751987195740728e-4_f64 * t38391 + 0.5107751987195740728e-4_f64 * t38393 + 0.1702583995731913576e-4_f64 * t38395 + 0.5107751987195740728e-4_f64 * t38398 + 0.2553875993597870364e-4_f64 * t38404 + 0.85129199786595678799e-5_f64 * t38412 + t42609 + 0.15323255961587222184e-3_f64 * t38420;
    t42611
}
