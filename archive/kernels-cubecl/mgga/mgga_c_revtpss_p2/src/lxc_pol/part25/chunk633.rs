//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 633/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk633<F: Float>(t247: F, t3639: F, t1264: F, t3368: F, t1230: F, t1260: F, t225: F, t3552: F, t480: F, t371: F, t482: F, t676: F) -> (F, F, F, F, F, F) {
    let t3640 = t247 * t3639;
    let t3643 = t1264 * t3368;
    let t3644 = t247 * t3643;
    let t3647 = t1230 * t1260;
    let t3650 = t3552 * t225;
    let t3651 = t3650 * t480;
    let t3655 = t371 * t676 * t482;
    (t3640, t3644, t3647, t3650, t3651, t3655)
}
