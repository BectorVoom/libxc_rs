//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 900/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk900<F: Float>(t1228: F, t3139: F, t1213: F, t2999: F, t89: F, t1186: F, t3704: F, t10696: F, t1240: F, t10478: F, t2770: F, t4246: F) -> (F, F, F, F, F, F) {
    let t55274 = t3139 * t1228;
    let t55558 = t89 * t2999 * t1213;
    let t55562 = t89 * t3704 * t1186;
    let t55768 = t1240 * t10696;
    let t55937 = t10478 * t1240;
    let t56098 = t2770 * t4246;
    (t55274, t55558, t55562, t55768, t55937, t56098)
}
