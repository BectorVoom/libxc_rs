//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1144/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1144(t36913: f64, t36916: f64, t36922: f64, t36925: f64, t36936: f64, t36948: f64, t38123: f64, t44004: f64, t44008: f64, t47840: f64, t47845: f64, t47855: f64, t47857: f64, t47861: f64, t47866: f64, t47868: f64, t47872: f64) -> f64 {
    let t49725 = t44004 - 0.638468998399467591e-4_f64 * t47840 + 0.1915406995198402773e-3_f64 * t47845 - t44008 + 0.72042316457491791901e-3_f64 * t36913 + 0.66211599834018861287e-4_f64 * t36916 - 0.38422568777328955681e-2_f64 * t36922 - 0.1440846329149835838e-2_f64 * t36925 - 0.72042316457491791901e-3_f64 * t36936 + t38123 + 0.20496175532535769483e-3_f64 * t36948 - 0.85129199786595678799e-5_f64 * t47855 + 0.2553875993597870364e-4_f64 * t47857 + 0.2553875993597870364e-4_f64 * t47861 + 0.1702583995731913576e-4_f64 * t47866 - 0.85129199786595678799e-5_f64 * t47868 - 0.212822999466489197e-4_f64 * t47872;
    t49725
}
