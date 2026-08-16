//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1009/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1009(t78244: f64, t25854: f64, t77786: f64, t27048: f64, t77789: f64, t27055: f64, t77341: f64, t41116: f64, t77345: f64, t75748: f64, t75756: f64, t71628: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78245 = 0.5987120850931904282e-1_f64 * t78244;
    let t78246 = t25854 * t77786;
    let t78247 = 0.8980681276397856423e-1_f64 * t78246;
    let t78248 = t27048 * t77789;
    let t78249 = 0.8980681276397856423e-1_f64 * t78248;
    let t78251 = 0.35922725105591425692e0_f64 * t27055 * t77341;
    let t78253 = 0.47896966807455234256e0_f64 * t41116 * t77345;
    let t78271 = 0.79808624799933448875e-4_f64 * t75748;
    let t78272 = 0.212822999466489197e-4_f64 * t75756;
    let t78273 = 0.39914139006212695213e-1_f64 * t71628;
    (t78245, t78247, t78249, t78251, t78253, t78271, t78272, t78273)
}
