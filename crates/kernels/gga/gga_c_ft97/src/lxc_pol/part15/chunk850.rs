//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 850/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk850<F: Float>(t18037: F, t3799: F, t13598: F, t21196: F, t701: F, t18034: F, t173: F, t21182: F, t21200: F, t18043: F, t3803: F, t18031: F, t21192: F, t21210: F, t227: F, t9: F) -> (F, F, F, F, F, F, F, F, F) {
    let t79759 = t3799 * t18037;
    let t79782 = t701 * t13598 * t21196;
    let t79786 = t3799 * t18034;
    let t79789 = t701 * t173 * t21182;
    let t79792 = t701 * t173 * t21200;
    let t79794 = t18043 * t3803;
    let t79796 = t3799 * t18031;
    let t79799 = t701 * t173 * t21192;
    let t79802 = t9 * t227 * t21210;
    (t79759, t79782, t79786, t79789, t79792, t79794, t79796, t79799, t79802)
}
