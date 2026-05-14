//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1262/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1262<F: Float>(t26976: F, t2823: F, t761: F, t8892: F, t2061: F, t3033: F, t6027: F, t6029: F, t26947: F, t7877: F, t27774: F, t6217: F, t277: F, t8629: F, t2670: F, t7234: F) -> (F, F, F, F, F, F, F, F) {
    let t28997 = t2823 * t26976;
    let t29002 = t8892 * t761;
    let t29003 = t2061 * t29002;
    let t29028 = t6027 * t3033 * t6029;
    let t29030 = t7877 * t26947;
    let t29054 = t6217 * t27774;
    let t29059 = t277 * t8629;
    let t29070 = t2670 * t7234;
    (t28997, t29002, t29003, t29028, t29030, t29054, t29059, t29070)
}
