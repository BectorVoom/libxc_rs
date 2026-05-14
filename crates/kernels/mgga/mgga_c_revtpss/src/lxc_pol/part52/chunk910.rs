//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 910/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk910<F: Float>(t31772: F, t4364: F, t886: F, t31767: F, t2769: F, t8648: F, t8476: F, t9645: F) -> (F, F, F, F) {
    let t31774 = t4364 * t31772 * t886;
    let t31775 = t31767 * t31774;
    let t31798 = t8648 * t2769;
    let t31805 = t8476 * t9645;
    (t31774, t31775, t31798, t31805)
}
