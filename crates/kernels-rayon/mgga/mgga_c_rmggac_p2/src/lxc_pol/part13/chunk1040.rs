//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1040/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1040(t38643: f64, t38645: f64, t38647: f64, t38675: f64, t34713: f64, t34717: f64, t37200: f64, t37201: f64, t37202: f64, t37203: f64, t38653: f64, t38658: f64, t38663: f64, t38678: f64, t38680: f64, t38685: f64, t38695: f64) -> f64 {
    let t42696 = 0.11918087970123395032e-3_f64 * t38643;
    let t42697 = 0.11918087970123395032e-3_f64 * t38645;
    let t42698 = 0.39726959900411316772e-4_f64 * t38647;
    let t42702 = 0.15965655602485078085e0_f64 * t38675;
    let t42709 = -t42696 + t42697 + t42698 - t37200 + 0.5107751987195740728e-4_f64 * t38653 - 0.5107751987195740728e-4_f64 * t38658 - 0.1702583995731913576e-4_f64 * t38663 - t42702 + 0.5987120850931904282e-1_f64 * t38678 + 0.11974241701863808564e0_f64 * t38680 + 0.11974241701863808564e0_f64 * t38685 - 0.17961362552795712846e0_f64 * t38695 + t37201 + t37202 - t37203 - 0.10248087766267884741e-3_f64 * t34713 + 0.1440846329149835838e-2_f64 * t34717;
    t42709
}
