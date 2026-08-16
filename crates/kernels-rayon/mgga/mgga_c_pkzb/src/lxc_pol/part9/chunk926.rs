//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 926/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk926(t1532: f64, t7046: f64, t5325: f64, t5328: f64, t5339: f64, t5025: f64, t5028: f64, t5040: f64, t5066: f64, t5069: f64, t5073: f64, t5186: f64, t5324: f64, t5333: f64, t5338: f64, t5344: f64, t7045: f64) -> (f64, f64, f64, f64, f64) {
    let t7047 = t7046 * t1532;
    let t7048 = 0.10843581300301739842e-1_f64 * t7047;
    let t7049 = 0.4883052614935078681e-3_f64 * t5325;
    let t7050 = 0.18311447306006545054e-3_f64 * t5328;
    let t7051 = 0.11696447245269292414e1_f64 * t5339;
    let t7052 = t5186 + t7045 + t5025 + t7048 + t5028 - t5324 + t5040 + t5066 - t5069 - t5073 + t7049 - t7050 + t5333 - t5338 - t7051 - t5344;
    (t7048, t7049, t7050, t7051, t7052)
}
