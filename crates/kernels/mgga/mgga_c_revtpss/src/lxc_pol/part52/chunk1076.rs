//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1076/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1076<F: Float>(t248: F, t33711: F, t125: F, t1579: F, t246: F, t244: F, t31838: F, t1561: F, t31846: F, t4450: F, t31851: F, t8486: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33712 = t33711 * t248;
    let t33714 = t125 * t1579;
    let t33715 = t246 * t33714;
    let t33716 = t244 * t33715;
    let t33717 = t31838 * t33716;
    let t33719 = t31846 * t1561;
    let t33721 = t246 * t4450;
    let t33722 = t31851 * t33721;
    let t33723 = t8486 * t33722;
    (t33712, t33714, t33715, t33716, t33717, t33719, t33721, t33722, t33723)
}
