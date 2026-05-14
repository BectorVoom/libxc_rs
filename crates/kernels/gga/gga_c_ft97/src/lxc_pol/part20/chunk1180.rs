//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1180/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1180<F: Float>(t317: F, t9577: F, t25462: F, t28935: F, t28951: F, t10683: F, t13863: F, t14075: F, t14116: F, t15200: F, t2360: F, t25413: F, t25446: F, t25459: F, t25465: F, t2665: F, t28938: F, t28939: F, t28940: F, t28941: F, t28944: F, t28947: F, t29000: F, t29002: F, t3051: F, t3746: F, t3886: F, t4162: F, t6209: F, t6216: F, t6261: F, t683: F, t880: F, t98273: F) -> (F,) {
    let t111783 = t317 * t9577;
    let t111795 = 2.0 / 27.0 * t25462 * t28935;
    let t111801 = 2.0 / 27.0 * t25462 * t28951;
    let t111802 = 2.0 * t6216 * t10683 * t25465 * t4162 + 2.0 / 9.0 * t29000 * t2665 * t25446 * t3746 + 2.0 / 9.0 * t6209 * t3051 * t29002 + 2.0 / 9.0 * t25459 * t28941 + 2.0 / 9.0 * t6216 * t683 * t6261 * t28940 + 2.0 / 9.0 * t6216 * t28938 * t880 * t2360 * t3886 + t6216 * t28938 * t28939 * t14075 / 9.0 + 2.0 / 9.0 * t6216 * t28944 * t111783 * t13863 - 4.0 / 9.0 * t29000 * t28938 * t28939 * t14116 - 2.0 / 27.0 * t25459 * t28947 - t111795 - t6216 * t98273 * t25413 * t15200 / 3.0 - t111801;
    (t111802,)
}
