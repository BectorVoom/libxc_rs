//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2544/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2544(t43959: f64, t4786: f64, t11424: f64, t15051: f64, t11185: f64, t15061: f64, t1117: f64, t14914: f64, t3264: f64, t11350: f64, t1682: f64, t1136: f64, t15118: f64, t3332: f64, t44131: f64, t51453: f64, t51456: f64, t51459: f64, t51463: f64, t51466: f64, t51470: f64, t51472: f64, t51474: f64, t51476: f64) -> (f64, f64, f64, f64, f64) {
    let t51478 = 0.48245938496077605201e2_f64 * t43959 * t4786;
    let t51480 = 12.0_f64 * t11424 * t15051;
    let t51482 = 0.96491876992155210402e2_f64 * t11185 * t15061;
    let t51485 = 6.0_f64 * t3264 * t14914 * t1117;
    let t51486 = t11350 * t1682;
    let t51493 = t51453 + t51456 - t51459 - t51463 - t51466 - t51470 + t51472 - t51474 + t51476 - t51478 + t51480 - t51482 + t51485 + 0.6207121550312808036e4_f64 * t51486 * t44131 * t1136 - 6.0_f64 * t3332 * t15118 * t1136;
    (t51478, t51480, t51482, t51485, t51493)
}
