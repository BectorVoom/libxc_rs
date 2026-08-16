//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1176/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1176(t1154: f64, t14915: f64, t829: f64, t10541: f64, t10544: f64, t10548: f64, t1153: f64, t14133: f64, t14210: f64, t14216: f64, t14238: f64, t14278: f64, t14283: f64, t14896: f64, t14899: f64, t14902: f64, t14907: f64, t14913: f64, t3295: f64, t3381: f64, t348: f64, t4602: f64, t4607: f64, t4638: f64, t4643: f64, t5111: f64) -> f64 {
    let t14917 = t1154 * t14915 * t829;
    let t14920 = -0.1857375e-1_f64 * t10544 * t4638 + 0.46434375e-2_f64 * t5111 * t14278 - 0.1857375e-1_f64 * t3381 * t14283 - 0.1857375e-1_f64 * t10544 * t4607 + 0.1857375e-1_f64 * t3381 * t14216 - 0.46434375e-2_f64 * t5111 * t14238 - t10541 + 0.24765e-1_f64 * t14896 * t4643 + 0.9286875e-2_f64 * t14899 * t4602 + 0.619125e-2_f64 * t14902 * t348 - 0.1857375e-1_f64 * t3381 * t14210 - 0.1857375e-1_f64 * t14907 * t3295 + 0.9286875e-2_f64 * t5111 * t14133 - 0.26531111111111111111e-1_f64 * t10548 - 0.35374814814814814814e-1_f64 * t14913 - 0.53062222222222222222e-1_f64 * t1153 * t14917;
    t14920
}
