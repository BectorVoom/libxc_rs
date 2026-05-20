//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3052/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3052<F: Float>(t10111: F, t22: F, t4518: F, t10871: F, t10952: F, t122: F, t1558: F, t2482: F, t2722: F, t676: F, t72: F, t231: F, t2782: F, t2783: F, t51306: F) -> (F, F, F) {
    let t51660 = t10111 * t4518 * t22;
    let t51668 = t2482 * t10952 * t1558 * t10871 * t72 * t122 * t676 * t2722;
    let t51672 = t2782 * t2783 * t51306 * t231;
    (t51660, t51668, t51672)
}
