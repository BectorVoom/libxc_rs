//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1238/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1238<F: Float>(t41694: F, t43495: F, t43497: F, t43500: F, t43503: F, t43506: F, t43509: F, t43512: F, t43514: F, t43516: F, t43518: F, t43521: F) -> F {
    let t44424 = -F::cast_from(0.87327386630866483588e-2_f64) * t43495 - F::cast_from(0.93149212406257582492e-1_f64) * t43497 + F::cast_from(0.17336443480108537126e0_f64) * t43500 - F::cast_from(0.46230515946956099003e0_f64) * t43503 + F::cast_from(0.17465477326173296718e-1_f64) * t43506 + F::cast_from(0.52396431978519890152e-1_f64) * t43509 + F::cast_from(0.32927245914677557992e0_f64) * t43512 + F::cast_from(0.34672886960217074252e0_f64) * t43514 + F::cast_from(0.52009330440325611378e0_f64) * t43516 + t41694 + F::cast_from(0.43663693315433241794e-2_f64) * t43518 - F::cast_from(0.46574606203128791246e-1_f64) * t43521;
    t44424
}
