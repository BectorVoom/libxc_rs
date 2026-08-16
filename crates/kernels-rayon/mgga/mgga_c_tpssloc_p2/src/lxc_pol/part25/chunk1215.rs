//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1215/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1215(t82153: f64, t218: f64, t24234: f64, t24325: f64, t259: f64, t2591: f64, t2597: f64, t7084: f64, t7092: f64, t798: f64, t82169: f64, t82172: f64, t82174: f64, t82179: f64, t82182: f64, t82209: f64, t84939: f64, t9593: f64) -> (f64, f64) {
    let t85101 = 0.27415567780803773942e-2_f64 * t82153;
    let t85126 = -0.3289868133696452873e-1_f64 * t82169 + 0.49348022005446793095e-1_f64 * t82172 + 0.46058153871750340221e0_f64 * t82174 + 0.9869604401089358619e-1_f64 * t82179 + 12.0_f64 * t9593 * t7092 + 3.0_f64 * t798 * t24234 * t259 + t218 * t84939 * t259 - 0.49348022005446793095e-1_f64 * t82182 + 12.0_f64 * t2597 * t24325 + 3.0_f64 * t2591 * t7084 * t259 - 0.76763589786250567036e0_f64 * t82209;
    (t85101, t85126)
}
