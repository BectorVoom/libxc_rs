//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 384/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk384<F: Float>(t1793: F, t581: F, t1432: F, t1720: F, t1509: F, t681: F, t153: F, t181: F, t101: F, t1302: F) -> (F, F, F, F, F) {
    let t1794 = t581 * t1793;
    let t1795 = t1720 * t1432;
    let t1798 = t681 * t1509;
    let t1799 = t153 * t1798;
    let t1800 = t181 * t1799;
    let t1803 = t101 * t1302;
    (t1794, t1795, t1798, t1800, t1803)
}
