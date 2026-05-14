//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 983/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk983<F: Float>(t21201: F, t3799: F, t1124: F, t79802: F, t18043: F, t5046: F, t5038: F, t41447: F, t420: F, t701: F, t88252: F, t1107: F, t207: F, t14: F, t228: F, t231: F) -> (F, F, F, F, F, F, F) {
    let t88575 = t3799 * t21201;
    let t88577 = t79802 * t1124;
    let t88579 = t18043 * t5046;
    let t88581 = t18043 * t5038;
    let t88585 = t701 * t420 * t41447 * t88252;
    let t88593 = 1.0 / t207 / t1107;
    let t88596 = t228 * t88593 * t14 * t231;
    (t88575, t88577, t88579, t88581, t88585, t88593, t88596)
}
