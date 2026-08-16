//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 946/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk946(t10583: f64, t3399: f64, t6272: f64, t1154: f64, t14915: f64, t1646: f64, t330: f64, t6478: f64, t829: f64, t10544: f64, t1110: f64, t1115: f64, t1143: f64, t1153: f64, t14940: f64, t14956: f64, t14959: f64, t1757: f64, t1761: f64, t1780: f64, t18547: f64, t18551: f64, t18740: f64, t18858: f64, t3381: f64, t365: f64, t4626: f64, t5102: f64, t5122: f64, t6593: f64, t6605: f64, t6641: f64) -> f64 {
    let t20076 = t3399 * t10583 * t6272;
    let t20080 = t1154 * t14915 * t1646;
    let t20084 = t6478 * t330;
    let t20086 = t1154 * t20084 * t829;
    let t20093 = -0.619125e-2_f64 * t1143 * t6605 - 0.619125e-2_f64 * t365 * t18858 - 0.232171875e-2_f64 * t14940 * t18740 - 0.619125e-2_f64 * t6641 * t1115 + 0.1857375e-1_f64 * t5102 * t1757 + 0.1857375e-1_f64 * t1780 * t4626 - 0.123825e-1_f64 * t5102 * t1761 + 0.9286875e-2_f64 * t6641 * t1110 - 0.1857375e-1_f64 * t10544 * t6593 - 0.44218518518518518518e-1_f64 * t1153 * t20076 - 0.53062222222222222222e-1_f64 * t1153 * t20080 + 0.70749629629629629628e-1_f64 * t14956 - t14959 - 0.26531111111111111111e-1_f64 * t1153 * t20086 + 0.1857375e-1_f64 * t3381 * t18547 - 0.371475e-1_f64 * t5122 * t18551;
    t20093
}
