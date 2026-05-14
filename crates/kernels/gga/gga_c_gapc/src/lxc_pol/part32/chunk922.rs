//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 922/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk922<F: Float>(t1038: F, t20594: F, t3712: F, t19586: F, t9260: F, t19510: F, t5964: F, t1648: F, t1839: F, t20198: F, t13790: F, t8676: F, t190: F, t5261: F, t1045: F, t505: F) -> (F, F, F, F, F, F, F) {
    let t26007 = t3712 * t1038 * t20594;
    let t26017 = t9260 * t1038 * t19586;
    let t26034 = t5964 * t19510;
    let t26102 = t1648 * t1839 * t1038 * t20198;
    let t26226 = t8676 * t13790;
    let t26312 = t5261 * t190;
    let t26331 = t1045 * t505;
    (t26007, t26017, t26034, t26102, t26226, t26312, t26331)
}
