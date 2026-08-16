//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2330/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2330(t7365: f64, t94490: f64, t1715: f64, t974: f64, t24847: f64, t24771: f64, t7999: f64, t15418: f64, t2127: f64, t221: f64, t27553: f64, t11877: f64, t11907: f64, t11914: f64, t15245: f64, t15429: f64, t24765: f64, t24834: f64, t24838: f64, t27406: f64, t27454: f64, t27546: f64, t7283: f64, t8082: f64, t8083: f64, t86073: f64, t86095: f64, t94588: f64) -> (f64, f64) {
    let t95758 = t94490 * t7365;
    let t95760 = t974 * t1715;
    let t95761 = t24847 * t95760;
    let t95768 = t7999 * t24771;
    let t95772 = t2127 * t221 * t15418;
    let t95774 = 0.24369393582936687948e-2_f64 * t95772 * t27553;
    let t95779 = -0.18277045187202515961e-2_f64 * t86073 - 0.82246703342411321825e-2_f64 * t7283 * t94588 * t27454 + 0.48738787165873375896e-2_f64 * t95758 - 0.16449340668482264365e-1_f64 * t95761 * t24834 - t15245 * t24838 + t11877 * t8083 + 0.43864908449286038306e-1_f64 * t27406 * t24765 + 0.48738787165873375895e-2_f64 * t95768 - 0.18277045187202515961e-2_f64 * t86095 + t95774 - 2.0_f64 * t11907 * t27546 + t11914 * t8082 * t15429;
    (t95772, t95779)
}
