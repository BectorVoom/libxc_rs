//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1872/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1872<F: Float>(t27679: F, t7145: F, t7828: F, t999: F, t7160: F, t1651: F, t7135: F, t7821: F, t1096: F, t25464: F, t1647: F, t1976: F) -> (F, F, F, F, F, F, F) {
    let t27680 = t7145 * t27679;
    let t27683 = t7828 * t999;
    let t27684 = t7160 * t27683;
    let t27687 = t7135 * t1651;
    let t27688 = t7145 * t27687;
    let t27691 = t7821 * t999;
    let t27692 = t7145 * t27691;
    let t27695 = t7828 * t1096;
    let t27696 = t25464 * t27695;
    let t27699 = t1647 * t1976;
    (t27680, t27684, t27687, t27688, t27692, t27696, t27699)
}
