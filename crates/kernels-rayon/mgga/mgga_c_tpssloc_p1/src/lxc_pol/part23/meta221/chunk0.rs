//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 869/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk869(t1714: f64, t4899: f64, t11545: f64, t60: f64, t461: f64, t11588: f64, t134: f64, t3439: f64, t15026: f64, t3032: f64, t3514: f64, t11147: f64, t11778: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15390 = t4899 * t1714;
    let t15394 = t60 * t11545;
    let t15395 = t15394 * t461;
    let t15402 = t11588 * t461;
    let t15418 = t134 * t3439;
    let t15419 = t15418 * t461;
    let t15437 = t15026 * t3032;
    let t15438 = t15437 * t3514;
    let t15453 = t11778 * t11147;
    (t15390, t15394, t15395, t15402, t15418, t15419, t15437, t15438, t15453)
}
