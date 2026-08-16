//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 891/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk891(t12664: f64, t15362: f64, t28594: f64, t7785: f64, t12705: f64, t7416: f64, t10012: f64, t2530: f64, t2684: f64, t9438: f64, t12657: f64, t23157: f64) -> (f64, f64, f64, f64, f64) {
    let t41305 = t15362 * t12664;
    let t41307 = t28594 * t7785;
    let t41312 = t7416 * t12705;
    let t41316 = t2684 * t9438 * t10012 * t2530;
    let t41330 = t23157 * t12657;
    (t41305, t41307, t41312, t41316, t41330)
}
