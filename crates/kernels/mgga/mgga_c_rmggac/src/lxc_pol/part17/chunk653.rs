//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 653/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk653<F: Float>(t10100: F, t236: F, t3352: F, t7230: F, t1707: F, t511: F, t3351: F, t1916: F, t687: F, t1704: F, t234: F, t681: F, t1953: F, t668: F, t72: F, t2073: F, t9877: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10101 = t236 * t10100;
    let t10102 = t3352 * t10101;
    let t10103 = t7230 * t10102;
    let t10104 = 0.31923449919973379548e-4 * t10103;
    let t10105 = t511 * t1707;
    let t10106 = t3352 * t10105;
    let t10107 = t3351 * t10106;
    let t10108 = 0.76616279807936110914e-4 * t10107;
    let t10110 = t1916 * t687;
    let t10111 = 0.19957069503106347607e-1 * t10110;
    let t10112 = t234 * t1704;
    let t10113 = t10112 * t681;
    let t10114 = 0.2993560425465952141e-1 * t10113;
    let t10115 = t1953 * t668;
    let t10116 = t72 * t10115;
    let t10120 = t2073 * t9877;
    (t10102, t10104, t10106, t10108, t10111, t10112, t10114, t10115, t10116, t10120)
}
