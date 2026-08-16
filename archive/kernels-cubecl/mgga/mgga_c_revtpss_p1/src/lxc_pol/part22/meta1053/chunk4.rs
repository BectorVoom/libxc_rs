//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3723/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3723<F: Float>(t12866: F, t5406: F, t58895: F, t17789: F, t21306: F, t17401: F, t17617: F, t15687: F, t17394: F, t3782: F, t1122: F, t5284: F) -> (F, F, F, F, F, F) {
    let t70612 = t12866 * t58895 * t5406;
    let t70616 = t21306 * t17789;
    let t70623 = t17401 * t17617;
    let t70629 = t17394 * t15687;
    let t70630 = t3782 * t70629;
    let t70633 = t1122 * t5284;
    (t70612, t70616, t70623, t70629, t70630, t70633)
}
