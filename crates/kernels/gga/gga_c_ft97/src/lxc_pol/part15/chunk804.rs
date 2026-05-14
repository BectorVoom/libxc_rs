//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 804/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk804<F: Float>(t10696: F, t1240: F, t10478: F, t2770: F, t4246: F, t1221: F, t3281: F, t1225: F, t1242: F, t89: F, t9555: F, t1250: F, t10491: F, t4423: F, t8232: F, t4426: F, t7773: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t55768 = t1240 * t10696;
    let t55937 = t10478 * t1240;
    let t56098 = t2770 * t4246;
    let t56264 = t3281 * t1221;
    let t56665 = t3281 * t1225;
    let t56689 = t89 * t9555 * t1242;
    let t56957 = t3281 * t1250;
    let t57089 = t10491 * t1240;
    let t57435 = t8232 * t4423;
    let t57491 = t89 * t7773 * t4426;
    (t55768, t55937, t56098, t56264, t56665, t56689, t56957, t57089, t57435, t57491)
}
