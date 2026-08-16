//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 668/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk668(t8543: f64, t8546: f64, t8549: f64, t8552: f64, t7438: f64, t8612: f64, t8617: f64, t8655: f64, t8669: f64, t8677: f64, t8822: f64, t8710: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9335 = 0.5987120850931904282e-1_f64 * t8543;
    let t9336 = 0.17961362552795712846e0_f64 * t8546;
    let t9337 = 0.35922725105591425692e0_f64 * t8549;
    let t9338 = 0.8980681276397856423e-1_f64 * t8552;
    let t9339 = 0.59590439850616975158e-4_f64 * t7438;
    let t9368 = 0.1064114997332445985e-4_f64 * t8612;
    let t9369 = 0.1064114997332445985e-4_f64 * t8617;
    let t9381 = 0.2993560425465952141e-1_f64 * t8655;
    let t9393 = 0.1064114997332445985e-4_f64 * t8669;
    let t9412 = 0.1064114997332445985e-4_f64 * t8677;
    let t9440 = 0.2993560425465952141e-1_f64 * t8822;
    let t9445 = 0.4838420607177634088e-3_f64 * t8710;
    (t9335, t9336, t9337, t9338, t9339, t9368, t9369, t9381, t9393, t9412, t9440, t9445)
}
