//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 727/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk727(t34709: f64, t7558: f64, t7349: f64, t7359: f64, t7760: f64, t7352: f64, t934: f64, t2010: f64, t7755: f64, t7197: f64, t892: f64, t7203: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34710 = t34709 * t7558;
    let t34711 = 0.65053455985619242968e-4_f64 * t34710;
    let t34713 = t7349 * t7359 * t7760;
    let t34715 = t934 * t7352;
    let t34717 = t2010 * t7755 * t34715;
    let t34724 = t892 * t7197;
    let t34735 = t892 * t7203;
    (t34711, t34713, t34715, t34717, t34724, t34735)
}
