//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 732/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk732<F: Float>(t20759: F, t2221: F, t1053: F, t4431: F, t2211: F, t2210: F, t20224: F, t3434: F, t3491: F, t4778: F, t91: F, t13119: F, t13123: F, t17214: F, t17249: F, t17250: F, t17251: F, t20536: F, t20540: F, t20551: F, t20666: F, t20669: F, t9383: F) -> (F, F, F, F, F, F, F, F) {
    let t20760 = t2221 * t20759;
    let t20763 = t4431 * t1053;
    let t20764 = t2211 * t20763;
    let t20765 = t2210 * t20764;
    let t20768 = t3434 * t20224;
    let t20769 = t2210 * t20768;
    let t20779 = t91 * t3491 * t4778;
    let t20781 = -t17214 + F::new(2.0) * t20666 - t20669 / F::new(9.0) - t13119 - t9383 - t13123 - F::new(10.0) / F::new(81.0) * t20536 - F::new(2.0) / F::new(3.0) * t20540 + F::new(4.0) / F::new(9.0) * t20551 - t20779 / F::new(4.0) + t17249 - t17250 + t17251;
    (t20760, t20763, t20764, t20765, t20768, t20769, t20779, t20781)
}
