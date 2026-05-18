//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 362/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk362<F: Float>(t1758: F, t534: F, t1757: F, t11: F, t14: F, t19: F, t20: F, t5: F, t195: F, t110: F, t518: F, t517: F) -> (F, F, F, F, F, F, F, F) {
    let t1759 = t1758 * t534;
    let t1761 = F::new(2.0) * t1757 * t1759;
    let t1764 = F::new(1.0) / t14 / t11 * t19;
    let t1765 = t20 * t5;
    let t1766 = t1765 * t195;
    let t1767 = t1764 * t1766;
    let t1769 = t518 * t110;
    let t1770 = t517 * t1769;
    (t1759, t1761, t1764, t1765, t1766, t1767, t1769, t1770)
}
