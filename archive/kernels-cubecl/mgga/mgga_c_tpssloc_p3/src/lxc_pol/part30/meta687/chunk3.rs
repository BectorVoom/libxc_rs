//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2180/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2180<F: Float>(t3886: F, t6439: F, t1307: F, t22633: F, t22635: F, t1985: F, t26193: F, t26202: F, t6888: F, t6891: F, t97511: F, t28116: F, t80650: F) -> (F, F, F, F) {
    let t97608 = t3886 * t6439;
    let t97611 = t22633 * t22635 * t97608 * t1307;
    let t97616 = t1985 * t26193 * t26202;
    let t97619 = t6888 * t97511 * t6891;
    let t97624 = t22633 * t80650 * t28116;
    (t97611, t97616, t97619, t97624)
}
