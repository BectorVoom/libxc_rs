//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1090/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1090<F: Float>(t466: F, t779: F, t2104: F, t2107: F, t5974: F, t5979: F, t5589: F, t735: F, t154: F, t276: F, t277: F, t4932: F) -> (F, F, F, F, F) {
    let t17867 = t466 * t779;
    let t17869 = t2104 * t17867 * t2107;
    let t17872 = t2104 * t5974 * t5979;
    let t17874 = t735 * t5589;
    let t17881 = F::cast_from(5.0_f64) / F::cast_from(486.0_f64) * t276 * t154 * t4932 * t277;
    (t17867, t17869, t17872, t17874, t17881)
}
