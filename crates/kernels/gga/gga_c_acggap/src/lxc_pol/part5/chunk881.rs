//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 881/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk881<F: Float>(t1008: F, t4667: F, t1106: F, t1181: F, t1586: F, t3391: F, t3730: F, t540: F, t1526: F, t3573: F, t13287: F, t13293: F, t1432: F, t3169: F, t1459: F, t171: F) -> (F, F, F, F, F, F) {
    let t15362 = t1008 * t4667;
    let t15366 = t3391 * t1181 * t1586 * t1106;
    let t15370 = t3391 * t1181 * t540 * t3730;
    let t15378 = t3573 * t1526;
    let t15384 = t13293 * t13287 * t1432 * t3169;
    let t15386 = t171 * t1459;
    (t15362, t15366, t15370, t15378, t15384, t15386)
}
