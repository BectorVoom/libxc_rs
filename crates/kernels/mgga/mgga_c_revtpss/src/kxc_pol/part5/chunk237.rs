//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 237/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk237<F: Float>(t779: F, t780: F, t689: F, t211: F) -> (F, F, F, F) {
    let t781 = t779 * t780;
    let t783 = F::cast_from(0.54878743191129263322e-2_f64) * t689 * t781;
    let t784 = t211 * t211;
    let t785 = F::new(1.0) / t784;
    (t781, t783, t784, t785)
}
