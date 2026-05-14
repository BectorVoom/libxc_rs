//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1126/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1126<F: Float>(t1111: F, t1165: F, t20764: F, t3391: F, t1101: F, t1899: F, t3361: F, t1181: F, t4643: F, t4718: F, t4521: F, t13084: F, t6343: F, t15905: F, t5855: F, t3382: F, t6086: F) -> (F, F, F, F, F, F, F) {
    let t23094 = t3391 * t1165 * t20764 * t1111;
    let t23098 = t3361 * t1165 * t1899 * t1101;
    let t23105 = t3391 * t1181 * t4643 * t4718;
    let t23109 = t3391 * t1181 * t4643 * t4521;
    let t23111 = t13084 * t6343;
    let t23113 = t15905 * t5855;
    let t23115 = t3382 * t6086;
    (t23094, t23098, t23105, t23109, t23111, t23113, t23115)
}
