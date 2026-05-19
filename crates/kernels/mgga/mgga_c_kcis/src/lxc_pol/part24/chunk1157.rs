//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1157/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1157<F: Float>(t7589: F, t92201: F, t92174: F, t7580: F, t92181: F, t91909: F, t91913: F, t91916: F, t91919: F, t91922: F, t92171: F, t92175: F, t92177: F, t92182: F, t92188: F, t92193: F, t92195: F, t92197: F) -> F {
    let t92202 = t7589 * t92201;
    let t92204 = t7589 * t92174;
    let t92206 = t7580 * t92181;
    let t92213 = -F::cast_from(0.55652820312500000001e-3_f64) * t92171 - F::cast_from(0.2782641015625e-3_f64) * t92175 + F::cast_from(0.97307291666666666666e-2_f64) * t92177 + F::cast_from(0.41703125000000000001e-2_f64) * t92182 + F::cast_from(0.41703125000000000001e-2_f64) * t92188 - F::cast_from(0.41703125000000000001e-2_f64) * t92193 + F::cast_from(0.55652820312500000001e-3_f64) * t92195 - F::cast_from(0.41703125000000000001e-2_f64) * t92197 - F::cast_from(0.208515625e-2_f64) * t92202 - F::cast_from(0.208515625e-2_f64) * t92204 + F::cast_from(0.83479230468750000001e-3_f64) * t92206 - F::cast_from(0.79593333333333333331e-1_f64) * t91909 - F::cast_from(0.59694999999999999999e-1_f64) * t91913 - F::cast_from(0.92858888888888888885e-1_f64) * t91916 - F::cast_from(0.39796666666666666665e-1_f64) * t91919 + F::cast_from(0.39796666666666666665e-1_f64) * t91922;
    t92213
}
