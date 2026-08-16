//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 852/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk852(t8369: f64, t8405: f64, t8408: f64, t8411: f64, t8414: f64, t8418: f64, t8423: f64, t8428: f64, t8433: f64, t8438: f64, t8444: f64, t8448: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42378 = 0.13637330827122670865e-1_f64 * t8369;
    let t42383 = 0.11974241701863808564e0_f64 * t8405;
    let t42384 = 0.17961362552795712846e0_f64 * t8408;
    let t42385 = 0.35922725105591425692e0_f64 * t8411;
    let t42386 = 0.11974241701863808564e0_f64 * t8414;
    let t42390 = 0.3405167991463827152e-4_f64 * t8418;
    let t42391 = 0.1702583995731913576e-4_f64 * t8423;
    let t42392 = 0.5107751987195740728e-4_f64 * t8428;
    let t42393 = 0.5107751987195740728e-4_f64 * t8433;
    let t42394 = 0.1702583995731913576e-4_f64 * t8438;
    let t42395 = 0.1702583995731913576e-4_f64 * t8444;
    let t42396 = 0.1702583995731913576e-4_f64 * t8448;
    (t42378, t42383, t42384, t42385, t42386, t42390, t42391, t42392, t42393, t42394, t42395, t42396)
}
