//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1008/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1008(t15394: f64, t461: f64, t11589: f64, t4904: f64, t3447: f64, t11588: f64, t4729: f64, t134: f64, t3439: f64, t4724: f64, t15026: f64, t3032: f64) -> (f64, f64, f64, f64, f64) {
    let t15395 = t15394 * t461;
    let t15399 = t11589 * t4904;
    let t15401 = 0.18518518518518518518e-3_f64 * t3447 * t15399;
    let t15402 = t11588 * t461;
    let t15403 = t15402 * t4729;
    let t15405 = 0.37037037037037037036e-3_f64 * t3447 * t15403;
    let t15418 = t134 * t3439;
    let t15419 = t15418 * t461;
    let t15420 = t15419 * t4724;
    let t15422 = 0.24691358024691358024e-3_f64 * t3447 * t15420;
    let t15437 = t15026 * t3032;
    (t15395, t15401, t15405, t15422, t15437)
}
