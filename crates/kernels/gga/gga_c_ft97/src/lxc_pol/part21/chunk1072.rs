//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1072/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1072<F: Float>(t358: F, t5743: F, t26379: F, t8392: F, t26383: F, t1786: F, t6524: F, t22943: F, t463: F, t1882: F, t26255: F, t26468: F, t26472: F, t26442: F, t26437: F, t1851: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t102783 = t5743 * t358;
    let t102836 = 4.0 / 9.0 * t8392 * t26379;
    let t102838 = 4.0 / 9.0 * t8392 * t26383;
    let t102848 = t1786 * t6524;
    let t102862 = t463 * t22943;
    let t102878 = 4.0 / 9.0 * t1882 * t26255;
    let t102880 = 4.0 / 9.0 * t1882 * t26468;
    let t102882 = 4.0 / 9.0 * t1882 * t26472;
    let t102903 = 4.0 / 81.0 * t8392 * t26442;
    let t102917 = 4.0 / 27.0 * t8392 * t26437;
    let t102921 = t6524 * t1851;
    (t102783, t102836, t102838, t102848, t102862, t102878, t102880, t102882, t102903, t102917, t102921)
}
