//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1077/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1077(t39289: f64, t42886: f64, t42890: f64, t42891: f64, t42892: f64, t42899: f64, t42906: f64, t45274: f64, t45277: f64, t45283: f64, t45285: f64, t45289: f64, t45291: f64, t45293: f64, t45295: f64, t45300: f64, t45305: f64, t45307: f64) -> f64 {
    let t48407 = -0.5107751987195740728e-4_f64 * t45274 + 0.212822999466489197e-4_f64 * t45277 + 0.1064114997332445985e-4_f64 * t45283 - 0.1702583995731913576e-4_f64 * t45285 - t42886 - t42890 + t42891 + 0.5454932330849068346e-1_f64 * t45289 - 0.40911992481368012595e-1_f64 * t45291 - t42892 + t42899 + 0.3405167991463827152e-4_f64 * t45293 - 0.5107751987195740728e-4_f64 * t45295 + t42906 - 0.79453919800822633544e-4_f64 * t39289 + 0.3192344991997337955e-4_f64 * t45300 + 0.1064114997332445985e-4_f64 * t45305 - 0.1064114997332445985e-4_f64 * t45307;
    t48407
}
