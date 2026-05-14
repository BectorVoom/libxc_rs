//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1078/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1078<F: Float>(t38617: F, t40070: F, t41668: F, t41669: F, t41670: F, t41671: F, t41672: F, t41682: F, t41687: F, t41689: F, t43488: F, t43490: F, t41694: F, t43495: F, t43497: F, t43500: F, t43503: F, t43506: F, t43509: F, t43512: F, t43514: F, t43516: F, t43518: F, t43521: F) -> (F, F) {
    let t44412 = t41668 + t41669 + t41670 + t41671 - t41672 + 0.34672886960217074252e0 * t43488 - 0.23804984598836975487e0 * t40070 + t41682 - t38617 + 0.19514881078765566037e-1 * t43490 - t41687 + t41689;
    let t44424 = -0.87327386630866483588e-2 * t43495 - 0.93149212406257582492e-1 * t43497 + 0.17336443480108537126e0 * t43500 - 0.46230515946956099003e0 * t43503 + 0.17465477326173296718e-1 * t43506 + 0.52396431978519890152e-1 * t43509 + 0.32927245914677557992e0 * t43512 + 0.34672886960217074252e0 * t43514 + 0.52009330440325611378e0 * t43516 + t41694 + 0.43663693315433241794e-2 * t43518 - 0.46574606203128791246e-1 * t43521;
    (t44412, t44424)
}
