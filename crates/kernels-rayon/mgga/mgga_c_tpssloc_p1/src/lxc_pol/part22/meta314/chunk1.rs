//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1493/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1493(t15402: f64, t4729: f64, t3447: f64, t134: f64, t3439: f64, t461: f64) -> (f64, f64, f64, f64) {
    let t15403 = t15402 * t4729;
    let t15405 = 0.37037037037037037036e-3_f64 * t3447 * t15403;
    let t15418 = t134 * t3439;
    let t15419 = t15418 * t461;
    (t15403, t15405, t15418, t15419)
}
