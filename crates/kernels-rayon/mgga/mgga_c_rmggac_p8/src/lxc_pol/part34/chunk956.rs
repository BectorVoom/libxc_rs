//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 956/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk956(t74267: f64, t74269: f64, t74272: f64, t74275: f64, t70905: f64, t74238: f64, t74243: f64, t76935: f64, t76937: f64, t76939: f64, t76940: f64, t76941: f64, t76942: f64, t76943: f64, t76946: f64, t76947: f64, t76948: f64) -> f64 {
    let t76949 = 0.16263363996404810741e-4_f64 * t74267;
    let t76950 = 0.38430329123504567781e-4_f64 * t74269;
    let t76951 = 0.16263363996404810741e-4_f64 * t74272;
    let t76952 = 0.72042316457491791901e-3_f64 * t74275;
    let t76953 = t76935 - 0.52557918278704101564e-6_f64 * t74238 + t76937 + 0.76860658247009135557e-5_f64 * t74243 - t76939 - t76940 + t76941 + t76942 - t70905 - t76943 - t76946 - t76947 - t76948 + t76949 + t76950 + t76951 - t76952;
    t76953
}
