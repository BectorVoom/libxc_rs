//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 885/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk885<F: Float>(t1568: F, t8477: F, t1579: F, t8471: F, t31812: F, t1558: F, t231: F, t31817: F, t1949: F, t7759: F, t8650: F, t8485: F, t248: F, t125: F, t246: F, t244: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t33695 = t8477 * t1568;
    let t33698 = t8471 * t1579;
    let t33699 = t31812 * t33698;
    let t33703 = t8471 * t1558 * t231;
    let t33704 = t31817 * t33703;
    let t33707 = t1949 * t7759;
    let t33708 = t8650 * t33707;
    let t33711 = t33695 * t8485;
    let t33712 = t33711 * t248;
    let t33714 = t125 * t1579;
    let t33715 = t246 * t33714;
    let t33716 = t244 * t33715;
    (t33695, t33698, t33699, t33703, t33704, t33707, t33708, t33711, t33712, t33714, t33715, t33716)
}
