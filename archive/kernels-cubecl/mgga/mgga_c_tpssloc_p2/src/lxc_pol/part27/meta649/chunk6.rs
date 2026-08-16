//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2256/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2256<F: Float>(t1649: F, t2553: F, t12971: F, t28: F, t1081: F, t4119: F, t13191: F, t25891: F, t25927: F, t57921: F, t13471: F, t1484: F, t3231: F) -> (F, F, F, F, F, F, F) {
    let t89881 = t1649 * t2553;
    let t89888 = t28 * t12971;
    let t89892 = t1081 * t4119;
    let t89896 = t25891 * t13191;
    let t89904 = t25927 * t57921;
    let t89907 = t28 * t13471;
    let t89911 = t3231 * t1484;
    (t89881, t89888, t89892, t89896, t89904, t89907, t89911)
}
