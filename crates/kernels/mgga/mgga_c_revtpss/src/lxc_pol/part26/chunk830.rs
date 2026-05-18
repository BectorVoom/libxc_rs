//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 830/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk830<F: Float>(t10638: F, t231: F, t2710: F, t2793: F, t9285: F, t2470: F, t2804: F, t874: F, t875: F, t9288: F, t251: F, t2722: F) -> (F, F, F, F, F) {
    let t10639 = t10638 * t231;
    let t10645 = F::new(0.46263278077393568556e-2) * t2710 * t2793 * t9285;
    let t10647 = t874 * t2804 * t2470;
    let t10651 = F::new(0.30356481678079769392e-1) * t874 * t875 * t9288;
    let t10652 = t251 * t2722;
    (t10639, t10645, t10647, t10651, t10652)
}
