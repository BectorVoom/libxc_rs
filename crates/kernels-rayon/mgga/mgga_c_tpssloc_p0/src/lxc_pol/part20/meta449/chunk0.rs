//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1899/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1899(t3378: f64, t4882: f64, t1164: f64, t3411: f64, t4879: f64, t11433: f64, t3396: f64, t4874: f64, t11424: f64, t4745: f64, t11185: f64, t4786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15036 = t4882 * t3378;
    let t15038 = 0.35089341735807877242e1_f64 * t1164 * t15036;
    let t15040 = 0.11696447245269292414e1_f64 * t3411 * t4879;
    let t15041 = t4882 * t11433;
    let t15043 = 0.17315859105681463759e2_f64 * t1164 * t15041;
    let t15044 = t4874 * t3396;
    let t15046 = 0.11696447245269292414e1_f64 * t1164 * t15044;
    let t15048 = 4.0_f64 * t11424 * t4745;
    let t15050 = 0.32163958997385070134e2_f64 * t11185 * t4786;
    (t15036, t15038, t15040, t15041, t15043, t15044, t15046, t15048, t15050)
}
