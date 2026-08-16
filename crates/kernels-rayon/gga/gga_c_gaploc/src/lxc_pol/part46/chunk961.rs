//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 961/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk961(t2963: f64, t3295: f64, t9796: f64, t40969: f64, t1029: f64, t9829: f64, t3431: f64, t5241: f64, t2679: f64, t9805: f64, t20671: f64, t28069: f64, t33148: f64) -> (f64, f64, f64, f64, f64) {
    let t43412 = t9796 * t2963 * t3295;
    let t43413 = 0.76685851907841499353e0_f64 * t43412;
    let t43414 = 0.38342925953920749676e1_f64 * t40969;
    let t43416 = t9796 * t1029 * t9829;
    let t43417 = 0.76685851907841499353e0_f64 * t43416;
    let t43419 = t5241 * t3431;
    let t43421 = t9805 * t43419 * t2679;
    let t43425 = t28069 * t20671 * t33148;
    (t43413, t43414, t43417, t43421, t43425)
}
