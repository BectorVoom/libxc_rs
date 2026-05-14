//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 624/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk624<F: Float>(t13797: F, t446: F, t10024: F, t13315: F, t13320: F, t2594: F, t3281: F, t13324: F, t2354: F, t1882: F, t3696: F, t3701: F, t13361: F, t13346: F, t724: F, t13352: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t13798 = t446 * t13797;
    let t13800 = t10024 * t13315;
    let t13801 = t446 * t13800;
    let t13803 = t2594 * t13320;
    let t13804 = t3281 * t13803;
    let t13806 = t2354 * t13324;
    let t13807 = t446 * t13806;
    let t13809 = t1882 * t3696;
    let t13810 = 2.0 / 27.0 * t13809;
    let t13811 = t1882 * t3701;
    let t13812 = 4.0 / 27.0 * t13811;
    let t13813 = t2354 * t13361;
    let t13814 = t446 * t13813;
    let t13816 = t724 * t13346;
    let t13817 = t446 * t13816;
    let t13819 = t2594 * t13352;
    (t13798, t13801, t13804, t13807, t13809, t13810, t13811, t13812, t13814, t13817, t13819)
}
