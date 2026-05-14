//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1072/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1072<F: Float>(t11708: F, t8240: F, t12538: F, t6395: F, t10868: F, t2147: F, t9292: F, t40103: F, t43495: F, t43497: F, t43500: F, t43503: F, t43506: F, t43509: F, t43512: F, t43514: F) -> (F,) {
    let t43516 = t8240 * t11708;
    let t43518 = t6395 * t12538;
    let t43521 = t2147 * t10868 * t9292;
    let t43523 = -0.43663693315433241792e-2 * t43495 - 0.46574606203128791245e-1 * t43497 + 0.86682217400542685632e-1 * t43500 - 0.23115257973478049502e0 * t43503 + 0.87327386630866483584e-2 * t43506 + 0.26198215989259945076e-1 * t43509 + 0.16463622957338778997e0 * t43512 + 0.17336443480108537126e0 * t43514 + 0.2600466522016280569e0 * t43516 + t40103 + 0.21831846657716620896e-2 * t43518 - 0.23287303101564395623e-1 * t43521;
    (t43523,)
}
