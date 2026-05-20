//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3008/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3008<F: Float>(t1062: F, t43154: F, t11202: F, t1651: F, t11940: F, t3105: F, t11923: F, t15926: F, t11922: F, t16016: F, t4899: F, t11994: F, t15734: F) -> (F, F, F, F, F, F) {
    let t54982 = t43154 * t1062;
    let t54983 = t1651 * t11202;
    let t54988 = t11940 * t3105;
    let t54991 = t15926 * t11923;
    let t54994 = t4899 * t11922 * t16016;
    let t55000 = t11994 * t15734;
    (t54982, t54983, t54988, t54991, t54994, t55000)
}
