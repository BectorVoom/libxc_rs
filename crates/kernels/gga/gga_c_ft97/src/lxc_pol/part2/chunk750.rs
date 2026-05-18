//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 750/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk750<F: Float>(t11076: F, t1808: F, t3119: F, t91: F, t1767: F, t8345: F, t965: F, t1766: F, t3157: F, t473: F, t11416: F, t11395: F, t11399: F, t11404: F, t11408: F, t11413: F, t8455: F) -> (F, F, F, F) {
    let t11781 = F::new(4.0) / F::new(27.0) * t11076;
    let t11783 = t91 * t3119 * t1808;
    let t11787 = t91 * t8345 * t965 * t1767;
    let t11789 = t1766 * t3157;
    let t11791 = t91 * t11789 * t473;
    let t11798 = F::new(4.0) / F::new(9.0) * t11416;
    let t11799 = -t11781 - t8455 - t11783 / F::new(12.0) + t11787 / F::new(8.0) - t11791 / F::new(6.0) - t11395 / F::new(3.0) - F::new(4.0) / F::new(9.0) * t11399 + F::new(22.0) / F::new(27.0) * t11404 + F::new(2.0) / F::new(3.0) * t11408 + F::new(4.0) / F::new(3.0) * t11413 - t11798;
    (t11783, t11787, t11791, t11799)
}
