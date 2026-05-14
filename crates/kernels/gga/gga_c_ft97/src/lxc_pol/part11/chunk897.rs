//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 897/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk897<F: Float>(t157: F, t40436: F, t604: F, t7763: F, t2101: F, t2142: F, t12709: F, t12714: F, t12723: F, t12724: F, t12982: F, t144: F, t1901: F, t1986: F, t2157: F, t2185: F, t2210: F, t2211: F, t2212: F, t38688: F, t38693: F, t38930: F, t40519: F, t40700: F, t40772: F, t446: F, t605: F, t609: F, t7745: F, t9017: F, t9145: F, t9362: F, t9432: F) -> (F,) {
    let t40926 = t40436 * t157;
    let t40931 = t604 * t7763;
    let t40945 = t2101 * t2142;
    let t40970 = -8.0 / 9.0 * t1901 * t12982 * t9362 + 40.0 / 81.0 * t1901 * t40926 * t12724 * t40700 + 40.0 / 81.0 * t1901 * t12723 * t40931 * t40772 - 20.0 / 27.0 * t1901 * t12723 * t12724 * t38930 + 4.0 / 9.0 * t1901 * t2210 * t2211 * t7745 * t609 - 8.0 / 3.0 * t1901 * t40945 * t9145 - 8.0 / 3.0 * t1901 * t12709 * t38688 * t2212 + 8.0 / 9.0 * t1901 * t12714 * t38693 * t2212 - 4.0 / 3.0 * t446 * t144 * t40519 - 4.0 * t446 * t2185 * t605 * t1986 * t2157 + 8.0 * t446 * t9432 * t605 * t9017 * t609;
    (t40970,)
}
