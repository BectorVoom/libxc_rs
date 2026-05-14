//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1068/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1068<F: Float>(t2252: F, t2644: F, t342: F, t784: F, t8639: F, t11401: F, t443: F, t444: F, t10051: F, t1160: F, t3951: F, t737: F, t265: F, t42109: F, t2486: F, t2568: F) -> (F, F, F, F, F, F, F) {
    let t44709 = t342 * t2252 * t2644;
    let t44716 = 5.0 / 54.0 * t342 * t8639 * t784;
    let t46862 = t443 * t444 * t11401;
    let t51340 = t1160 * t10051;
    let t51609 = t737 * t3951;
    let t51669 = t42109 * t265;
    let t51687 = t2486 * t2568;
    (t44709, t44716, t46862, t51340, t51609, t51669, t51687)
}
