//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 948/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk948<F: Float>(t1087: F, t2254: F, t6: F, t6851: F, t6172: F, t7418: F, t8131: F, t8141: F, t967: F, t2315: F, t2553: F, t7592: F, t7593: F, t2776: F, t286: F, t8132: F) -> (F, F, F, F, F, F, F, F) {
    let t15843 = t1087 * t2254;
    let t15853 = t6851 * t6;
    let t15884 = t7418 * t6172;
    let t15938 = t8131 * t967 * t8141;
    let t16133 = t2553 * t2315;
    let t16152 = t7592 * t7593 * t6;
    let t16181 = t2776 * t286;
    let t16182 = t8132 * t16181;
    (t15843, t15853, t15884, t15938, t16133, t16152, t16181, t16182)
}
