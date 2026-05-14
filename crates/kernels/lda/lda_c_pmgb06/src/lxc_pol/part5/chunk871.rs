//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 871/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk871<F: Float>(t5211: F, t6372: F, t2497: F, t3226: F, t2501: F, t3220: F, t443: F, t6225: F, t1447: F, t6387: F, t6391: F, t517: F, t6130: F, t1423: F, t6413: F, t6416: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15895 = t5211 * t6372;
    let t15897 = t3226 * t2497;
    let t15899 = t3220 * t2501;
    let t15935 = t6225 * t443;
    let t15943 = t1447 * t6387;
    let t15945 = t1447 * t6391;
    let t15947 = t6130 * t517;
    let t16029 = t1423 * t6413;
    let t16031 = t1423 * t6416;
    (t15895, t15897, t15899, t15935, t15943, t15945, t15947, t16029, t16031)
}
