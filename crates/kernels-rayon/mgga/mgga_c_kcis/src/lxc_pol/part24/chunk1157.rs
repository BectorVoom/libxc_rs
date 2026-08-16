//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1157/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1157(t7589: f64, t92201: f64, t92174: f64, t7580: f64, t92181: f64, t91909: f64, t91913: f64, t91916: f64, t91919: f64, t91922: f64, t92171: f64, t92175: f64, t92177: f64, t92182: f64, t92188: f64, t92193: f64, t92195: f64, t92197: f64) -> f64 {
    let t92202 = t7589 * t92201;
    let t92204 = t7589 * t92174;
    let t92206 = t7580 * t92181;
    let t92213 = -0.55652820312500000001e-3_f64 * t92171 - 0.2782641015625e-3_f64 * t92175 + 0.97307291666666666666e-2_f64 * t92177 + 0.41703125000000000001e-2_f64 * t92182 + 0.41703125000000000001e-2_f64 * t92188 - 0.41703125000000000001e-2_f64 * t92193 + 0.55652820312500000001e-3_f64 * t92195 - 0.41703125000000000001e-2_f64 * t92197 - 0.208515625e-2_f64 * t92202 - 0.208515625e-2_f64 * t92204 + 0.83479230468750000001e-3_f64 * t92206 - 0.79593333333333333331e-1_f64 * t91909 - 0.59694999999999999999e-1_f64 * t91913 - 0.92858888888888888885e-1_f64 * t91916 - 0.39796666666666666665e-1_f64 * t91919 + 0.39796666666666666665e-1_f64 * t91922;
    t92213
}
