//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 997/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk997<F: Float>(t1651: F, t8507: F, t31892: F, t1678: F, t8513: F, t1695: F) -> (F, F, F, F) {
    let t33796 = t8507 * t1651;
    let t33797 = t31892 * t33796;
    let t33800 = t8513 * t1678;
    let t33803 = t8507 * t1695;
    (t33796, t33797, t33800, t33803)
}
