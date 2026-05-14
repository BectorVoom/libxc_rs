//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 301/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk301<F: Float>(t1243: F, t1255: F, t1263: F, t1272: F, t1251: F, t1259: F, t1268: F, t1275: F) -> (F, F, F, F, F) {
    let t1389 = 2.0 * t1243;
    let t1391 = 0.6666666666666666 * t1255;
    let t1393 = 0.505765839233979 * t1263;
    let t1395 = 0.168588613077993 * t1272;
    let t1397 = t1389 - 2.0 * t1251 + t1391 + 2.0 * t1259 + t1393 - 0.505765839233979 * t1268 + t1395 + 0.505765839233979 * t1275;
    (t1389, t1391, t1393, t1395, t1397)
}
