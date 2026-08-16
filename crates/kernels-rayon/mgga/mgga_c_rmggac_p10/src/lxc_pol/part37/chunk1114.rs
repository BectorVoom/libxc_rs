//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1114/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1114(t76477: f64, t118: f64, t72088: f64, t76476: f64, t78222: f64, t78225: f64, t78228: f64, t78237: f64, t78240: f64, t78245: f64, t78247: f64, t78249: f64, t78251: f64, t78253: f64, t80163: f64) -> f64 {
    let t80496 = 0.49700494569958178262e-1_f64 * t76477;
    let t80497 = -0.39914139006212695214e-1_f64 * t118 * t80163 - t76476 - t78222 + t78225 + t80496 + t78228 - t72088 + t78237 - t78240 + t78245 - t78247 - t78249 - t78251 - t78253;
    t80497
}
