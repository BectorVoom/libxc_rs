//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1072/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1072(t34810: f64, t37228: f64, t42785: f64, t44986: f64, t44990: f64, t44994: f64, t44997: f64, t45002: f64, t45004: f64, t45012: f64, t45018: f64, t45020: f64, t45026: f64, t45032: f64, t45038: f64, t45044: f64, t45048: f64) -> f64 {
    let t48324 = -0.30646511923174444368e-3_f64 * t44986 + 0.61293023846348888736e-3_f64 * t44990 + 0.15323255961587222184e-3_f64 * t44994 - 0.15323255961587222184e-3_f64 * t44997 + 0.638468998399467591e-4_f64 * t45002 + 0.5107751987195740728e-4_f64 * t45004 + 0.5107751987195740728e-4_f64 * t45012 + 0.5107751987195740728e-4_f64 * t45018 + 0.1702583995731913576e-4_f64 * t45020 + 0.1702583995731913576e-4_f64 * t45026 + 0.1702583995731913576e-4_f64 * t45032 - t37228 - 0.66671395154821946452e-1_f64 * t34810 + 0.85129199786595678799e-5_f64 * t45038 - 0.85129199786595678799e-5_f64 * t45044 - 0.85129199786595678799e-5_f64 * t45048 - t42785;
    t48324
}
