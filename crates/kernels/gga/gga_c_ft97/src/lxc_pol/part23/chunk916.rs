//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 916/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk916<F: Float>(t242: F, t27925: F, t27911: F, t3977: F, t6166: F, t729: F, t1449: F, t3821: F, t762: F, t2469: F, t6921: F, t6940: F, t713: F, t27984: F, t2574: F, t265: F, t27878: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28236 = t242 * t27925;
    let t28239 = t242 * t27911;
    let t28243 = t729 * t3977 * t6166;
    let t28246 = t1449 * t3821;
    let t28248 = t729 * t762 * t28246;
    let t28252 = t729 * t2469 * t6921;
    let t28255 = t6940 * t713;
    let t28257 = t729 * t762 * t28255;
    let t28260 = t242 * t27984;
    let t28264 = t2574 * t265 * t27878;
    (t28236, t28239, t28243, t28246, t28248, t28252, t28255, t28257, t28260, t28264)
}
