//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 787/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk787<F: Float>(t67: F, t8063: F, t9: F, t425: F, t1732: F, t8130: F, t1736: F, t7763: F, t37357: F, t419: F, t420: F, t1725: F, t8098: F, t1743: F, t626: F, t8115: F) -> (F, F, F, F, F, F) {
    let t37784 = t9 * t67 * t8063;
    let t37785 = t37784 * t425;
    let t37787 = t8130 * t1732;
    let t37789 = t1736 * t7763;
    let t37792 = t419 * t420 * t37789 * t37357;
    let t37795 = t1725 * t8098;
    let t37798 = t419 * t626 * t1743;
    let t37800 = t1725 * t8115;
    (t37785, t37787, t37792, t37795, t37798, t37800)
}
