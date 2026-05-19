//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 900/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk900<F: Float>(t6006: F, t6008: F, t6013: F, t6017: F, t6021: F, t6023: F, t6025: F, t6030: F, t6032: F, t6035: F, t6039: F, t6042: F, t6045: F) -> F {
    let t6255 = -F::new(0.25e0) * t6006 - F::cast_from(0.13489583333333333333e-1_f64) * t6008 - F::new(0.20234375e-1) * t6013 - F::new(0.9375e-1) * t6017 - F::cast_from(0.101171875e-1_f64) * t6021 + F::new(0.625e-1) * t6023 + F::cast_from(0.53958333333333333333e-1_f64) * t6025 + F::new(0.1875e0) * t6030 + F::new(0.625e-1) * t6032 - F::cast_from(0.53958333333333333333e-1_f64) * t6035 - F::new(0.9375e-1) * t6039 - F::cast_from(0.16666666666666666667e0_f64) * t6042 + F::new(0.25e0) * t6045;
    t6255
}
