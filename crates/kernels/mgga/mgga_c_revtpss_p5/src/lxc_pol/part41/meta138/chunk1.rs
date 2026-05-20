//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 653/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk653<F: Float>(t3303: F, t471: F, t498: F, t1330: F, t72: F, t757: F, t530: F, t566: F, t525: F, t527: F, t2608: F, t520: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3783 = t3303 * t471;
    let t3800 = t498 * t498;
    let t3801 = F::new(1.0) / t3800;
    let t3825 = t1330 * t72;
    let t3826 = t3825 * t757;
    let t3828 = t530 * t566;
    let t3833 = F::new(1.0) / t525;
    let t3841 = F::new(1.0) / t527;
    let t3853 = t520 * t2608;
    (t3783, t3800, t3801, t3825, t3826, t3828, t3833, t3841, t3853)
}
