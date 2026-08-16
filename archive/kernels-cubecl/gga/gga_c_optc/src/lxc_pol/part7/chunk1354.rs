//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1354/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1354<F: Float>(t26881: F, t3126: F, t1028: F, t9123: F, t1135: F, t1121: F, t1128: F, t8907: F, t3145: F, t8428: F, t22035: F, t894: F) -> (F, F, F, F, F) {
    let t26954 = t26881 * t3126;
    let t26977 = t9123 * t1028;
    let t26981 = t1135 * t3126;
    let t26987 = t1121 * t1128 * t8907;
    let t26989 = t3145 * t8428;
    let t26991 = t894 * t26989 * t22035;
    (t26954, t26977, t26981, t26987, t26991)
}
