//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1062/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1062(t78248: f64, t27055: f64, t77341: f64, t41116: f64, t77345: f64, t118: f64, t305: f64, t326: f64, t333: f64, t5266: f64, t72088: f64, t76477: f64, t77233: f64, t77638: f64, t77816: f64, t77999: f64, t78228: f64, t78237: f64, t78240: f64, t78245: f64, t78247: f64) -> f64 {
    let t78249 = 0.8980681276397856423e-1_f64 * t78248;
    let t78251 = 0.35922725105591425692e0_f64 * t27055 * t77341;
    let t78253 = 0.47896966807455234256e0_f64 * t41116 * t77345;
    let t78254 = 0.49700494569958178264e-1_f64 * t76477 + t78228 - t72088 - 0.39914139006212695214e-1_f64 * t118 * t77638 - 0.59871208509319042821e-1_f64 * t326 * t77233 + 0.59871208509319042821e-1_f64 * t305 * t77816 + t78237 - t78240 + 0.11974241701863808564e0_f64 * t5266 * t77999 * t333 + t78245 - t78247 - t78249 - t78251 - t78253;
    t78254
}
