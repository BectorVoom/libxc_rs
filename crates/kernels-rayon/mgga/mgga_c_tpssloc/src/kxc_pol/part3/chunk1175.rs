//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1175/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1175(t15403: f64, t3447: f64, t14736: f64, t4900: f64, t14740: f64, t14731: f64, t11575: f64, t4904: f64, t134: f64, t3439: f64, t461: f64, t4724: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15405 = 0.37037037037037037036e-3_f64 * t3447 * t15403;
    let t15406 = t4900 * t14736;
    let t15409 = t4900 * t14740;
    let t15412 = t4900 * t14731;
    let t15415 = t11575 * t4904;
    let t15418 = t134 * t3439;
    let t15419 = t15418 * t461;
    let t15420 = t15419 * t4724;
    (t15405, t15406, t15409, t15412, t15415, t15420)
}
