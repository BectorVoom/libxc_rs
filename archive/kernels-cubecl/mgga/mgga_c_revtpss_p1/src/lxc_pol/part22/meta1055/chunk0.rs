//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3730/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3730<F: Float>(t11249: F, t1248: F, t1284: F, t20849: F, t3624: F, t12772: F, t17729: F, t21036: F, t3625: F, t44250: F, t6639: F, t17423: F, t21049: F) -> (F, F, F, F, F) {
    let t70794 = t11249 * t1248;
    let t70800 = t20849 * t1284 * t3624;
    let t70806 = t17729 * t12772 * t21036;
    let t70809 = t3625 * t44250 * t6639;
    let t70811 = t21049 * t17423;
    (t70794, t70800, t70806, t70809, t70811)
}
