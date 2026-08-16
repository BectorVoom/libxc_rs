//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1614/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1614(t1137: f64, t15117: f64, t1147: f64, t4832: f64, t1687: f64, t3400: f64, t1156: f64, t14829: f64, t3375: f64, t1129: f64, t11356: f64, t1148: f64, t1157: f64, t14840: f64, t14847: f64, t14849: f64, t14852: f64, t1695: f64, t3371: f64, t3378: f64, t3396: f64, t3404: f64, t4835: f64, t4858: f64) -> f64 {
    let t15118 = t15117 * t1137;
    let t15121 = t4832 * t1147;
    let t15126 = t1687 * t3400;
    let t15133 = t14829 * t1156;
    let t15136 = t1687 * t3375;
    let t15139 = 1.0_f64 * t1129 * t15118 + 0.11696447245269292414e1_f64 * t15121 * t1157 + 0.5848223622634646207e0_f64 * t4835 * t3396 + 0.17315859105681463759e2_f64 * t15126 * t3404 + 0.5848223622634646207e0_f64 * t11356 * t1695 + 0.11696447245269292414e1_f64 * t3371 * t4858 + 0.5848223622634646207e0_f64 * t1148 * t15133 + t14840 - 0.11696447245269292414e1_f64 * t15136 * t3378 - t14847 - t14849 - t14852;
    t15139
}
