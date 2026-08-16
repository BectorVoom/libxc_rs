//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 931/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk931(t2320: f64, t38374: f64, t1953: f64, t2127: f64, t39406: f64, t39452: f64, t45466: f64, t45469: f64, t45473: f64, t45477: f64, t45482: f64, t45484: f64, t45486: f64, t45488: f64, t45493: f64, t45495: f64, t45499: f64, t45503: f64, t45505: f64, t45507: f64, t72: f64) -> f64 {
    let t45509 = t38374 * t2320;
    let t45511 = t39406 + 0.20455996240684006296e-1_f64 * t45466 + 0.20455996240684006296e-1_f64 * t45469 + t72 * t1953 * t2127 - 0.24829349937757072983e-4_f64 * t45473 - 0.42564599893297839398e-5_f64 * t45477 + 0.53205749866622299248e-5_f64 * t45482 + 0.85129199786595678796e-5_f64 * t45484 - 0.24829349937757072983e-4_f64 * t45486 - 0.19863479950205658386e-4_f64 * t45488 + 0.76616279807936110914e-4_f64 * t45493 + 0.25538759935978703638e-4_f64 * t45495 - 0.1064114997332445985e-4_f64 * t45499 - 0.25538759935978703638e-4_f64 * t45503 + t39452 + 0.99317399751028291929e-5_f64 * t45505 - 0.1064114997332445985e-4_f64 * t45507 - 0.1064114997332445985e-4_f64 * t45509;
    t45511
}
