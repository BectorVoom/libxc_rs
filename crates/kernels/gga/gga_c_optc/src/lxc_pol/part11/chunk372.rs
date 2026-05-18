//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 372/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk372<F: Float>(t509: F, t603: F, t1796: F, t508: F, t565: F, t564: F, t67: F, t62: F, t571: F) -> (F, F, F, F, F, F, F) {
    let t1797 = t509 * t603;
    let t1799 = F::new(0.10843580882781524214e-1) * t1796 * t1797;
    let t1803 = t508 * t565;
    let t1807 = t564 * t67;
    let t1808 = F::new(1.0) / t1807;
    let t1809 = t62 * t1808;
    let t1810 = t571 * t571;
    (t1797, t1799, t1803, t1807, t1808, t1809, t1810)
}
