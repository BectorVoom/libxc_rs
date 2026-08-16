//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2055/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2055<F: Float>(t2122: F, t92569: F, t25163: F, t7575: F, t92576: F, t92584: F, t45958: F, t7565: F, t10301: F, t26754: F, t2247: F, t26781: F, t38: F) -> (F, F, F, F, F, F, F) {
    let t96752 = t2122 * t92569;
    let t96757 = t7575 * t25163;
    let t96760 = t2122 * t92576;
    let t96765 = t2122 * t92584;
    let t96773 = t45958 * t7565;
    let t96776 = t10301 * t26754;
    let t96792 = t2247 * t38 * t26781;
    (t96752, t96757, t96760, t96765, t96773, t96776, t96792)
}
