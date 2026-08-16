//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 376/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk376(t118: f64, t2228: f64, t2055: f64, t2058: f64, t2062: f64, t2071: f64, t2076: f64, t2082: f64, t2200: f64, t2204: f64, t2206: f64, t2209: f64, t2213: f64) -> f64 {
    let t2229 = t118 * t2228;
    let t2231 = 0.5987120850931904282e-1_f64 * t2055 - 0.8980681276397856423e-1_f64 * t2058 - 0.2993560425465952141e-1_f64 * t2062 - t2200 - 0.20455996240684006298e-1_f64 * t2071 + 0.2727466165424534173e-1_f64 * t2076 + 0.68186654135613354325e-2_f64 * t2082 + t2204 + 0.59871208509319042821e-1_f64 * t2206 - 0.59871208509319042821e-1_f64 * t2209 - 0.39914139006212695214e-1_f64 * t2213 + 0.19957069503106347607e-1_f64 * t2229;
    t2231
}
