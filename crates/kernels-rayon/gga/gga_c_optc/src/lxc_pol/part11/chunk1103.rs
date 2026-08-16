//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1103/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1103(t11603: f64, t5257: f64, t36863: f64, t5228: f64, t4297: f64, t190: f64, t3086: f64, t136: f64, t1220: f64, t5232: f64, t7274: f64, t23: f64, t5236: f64, t5238: f64, t5241: f64) -> (f64, f64, f64, f64, f64) {
    let t43571 = t5257 * t11603;
    let t43583 = t36863 * t5228;
    let t43584 = t4297 * t43583;
    let t43635 = t3086 * t190;
    let t43636 = t43635 * t136;
    let t43649 = t1220 * t7274 * t5232;
    let t43671 = t5236 * t5238 * t5241 * t23;
    (t43571, t43584, t43636, t43649, t43671)
}
