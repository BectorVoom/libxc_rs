//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 988/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk988<F: Float>(t3817: F, t679: F, t689: F, t17863: F, t2418: F, t1127: F, t2378: F, t2395: F, t1689: F, t3771: F, t6813: F, t1109: F, t17840: F, t2393: F, t17859: F, t25: F, t3762: F) -> (F, F, F, F, F, F, F) {
    let t65763 = t3817 * t679 * t689;
    let t66066 = t17863 * t2418;
    let t66071 = t1127 * t2378 * t2395;
    let t66076 = t3771 * t6813 * t1689;
    let t66088 = t17840 * t1109 * t2393 * t2395;
    let t66105 = t1127 * t2393 * t2395;
    let t66121 = t17859 * t25 * t3762;
    (t65763, t66066, t66071, t66076, t66088, t66105, t66121)
}
