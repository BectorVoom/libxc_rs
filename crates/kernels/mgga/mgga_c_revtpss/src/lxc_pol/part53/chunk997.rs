//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 997/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk997<F: Float>(t31798: F, t31805: F, t31801: F, t8477: F, t860: F, t11007: F, t822: F) -> (F, F, F, F) {
    let t31806 = t31805 * t31798;
    let t31808 = F::new(0.25389723392137995738e-1) * t31806 * t31801;
    let t31809 = t8477 * t860;
    let t31812 = t11007 * t822;
    (t31806, t31808, t31809, t31812)
}
