//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1024/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1024<F: Float>(t14400: F, t14402: F, t14514: F, t14516: F, t14519: F, t14522: F, t14525: F, t14529: F, t14532: F, t14536: F, t14538: F, t14541: F, t14543: F, t14548: F, t14552: F) -> F {
    let t15149 = -F::cast_from(0.28125e0_f64) * t14400 + F::cast_from(0.303515625e-1_f64) * t14402 + F::cast_from(0.9375e-1_f64) * t14514 - F::cast_from(0.13489583333333333333e-1_f64) * t14516 + F::cast_from(0.40468749999999999999e-1_f64) * t14519 - F::cast_from(0.62499999999999999999e-1_f64) * t14522 + F::cast_from(0.75e0_f64) * t14525 + F::cast_from(0.625e-1_f64) * t14529 - F::cast_from(0.60703125e-1_f64) * t14532 - F::cast_from(0.13489583333333333333e-1_f64) * t14536 - F::cast_from(1.0_f64) * t14538 - F::cast_from(0.50000000000000000001e0_f64) * t14541 + F::cast_from(0.1875e0_f64) * t14543 + F::cast_from(0.60703125e-1_f64) * t14548 - F::cast_from(0.101171875e-1_f64) * t14552;
    t15149
}
