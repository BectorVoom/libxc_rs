//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1475/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1475<F: Float>(t3625: F, t44250: F, t6639: F, t21439: F, t3624: F, t11249: F, t6622: F, t3682: F, t6667: F, t474: F, t6593: F, t3089: F) -> (F, F, F, F, F) {
    let t70809 = t3625 * t44250 * t6639;
    let t70819 = t21439 * t3624;
    let t70890 = t6622 * t11249;
    let t70942 = t6667 * t3682;
    let t70993 = t474 * t6593;
    let t70994 = t70993 * t3089;
    (t70809, t70819, t70890, t70942, t70994)
}
