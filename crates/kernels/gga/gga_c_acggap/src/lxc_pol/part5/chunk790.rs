//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 790/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk790<F: Float>(t277: F, t2977: F, t775: F, t761: F, t778: F, t13: F, t2: F, t3151: F, t3157: F, t721: F, t228: F, t2670: F, t163: F, t661: F, t660: F, t203: F, t985: F) -> (F, F, F, F, F, F, F, F) {
    let t11780 = 480.0 * t2977 * t277;
    let t11784 = t775 * t775;
    let t11787 = t761 * t761;
    let t11788 = t778 * t778;
    let t11792 = 0.24955700379505800916e5 * t13 / t11784 * t11787 / t11788;
    let t11795 = t3157 * t2 * t3151 * t721;
    let t11797 = t2670 * t228;
    let t11799 = t661 * t163;
    let t11800 = t660 * t11799;
    let t11802 = t203 * t985;
    (t11780, t11787, t11792, t11795, t11797, t11799, t11800, t11802)
}
