//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3390/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3390<F: Float>(t1634: F, t52877: F, t63597: F, t11299: F, t2875: F, t6110: F, t15101: F, t15383: F, t63633: F, t63636: F, t63638: F, t63641: F, t63644: F, t63647: F, t63649: F, t63653: F, t63656: F, t63660: F, t63662: F) -> (F, F, F, F) {
    let t63665 = F::cast_from(0.14035736694323150897e2_f64) * t52877 * t1634 * t63597;
    let t63668 = F::new(24.0) * t11299 * t6110 * t2875;
    let t63670 = F::new(4.0) * t15101 * t15383;
    let t63671 = -t63633 - t63636 - t63638 - t63641 - t63644 - t63647 + t63649 + t63653 - t63656 + t63660 - t63662 - t63665 - t63668 - t63670;
    (t63665, t63668, t63670, t63671)
}
