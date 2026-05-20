//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1492/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1492<F: Float>(t1464: F, t8330: F, t31204: F, t575: F, t1455: F, t8349: F, t31244: F, t571: F, t2212: F, t4153: F, t10199: F, t2195: F) -> (F, F, F, F, F, F) {
    let t117153 = t8330 * t1464;
    let t117155 = t31204 * t575;
    let t117161 = t1455 * t8349;
    let t117168 = t571 * t31244;
    let t117170 = t4153 * t2212;
    let t117183 = F::new(154.0) / F::new(27.0) * t10199 * t2195;
    (t117153, t117155, t117161, t117168, t117170, t117183)
}
