//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1217/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1217<F: Float>(t10868: F, t2147: F, t9292: F, t40103: F, t43495: F, t43497: F, t43500: F, t43503: F, t43506: F, t43509: F, t43512: F, t43514: F, t43516: F, t43518: F) -> F {
    let t43521 = t2147 * t10868 * t9292;
    let t43523 = -F::cast_from(0.43663693315433241792e-2_f64) * t43495 - F::cast_from(0.46574606203128791245e-1_f64) * t43497 + F::cast_from(0.86682217400542685632e-1_f64) * t43500 - F::cast_from(0.23115257973478049502e0_f64) * t43503 + F::cast_from(0.87327386630866483584e-2_f64) * t43506 + F::cast_from(0.26198215989259945076e-1_f64) * t43509 + F::cast_from(0.16463622957338778997e0_f64) * t43512 + F::cast_from(0.17336443480108537126e0_f64) * t43514 + F::cast_from(0.2600466522016280569e0_f64) * t43516 + t40103 + F::cast_from(0.21831846657716620896e-2_f64) * t43518 - F::cast_from(0.23287303101564395623e-1_f64) * t43521;
    t43523
}
