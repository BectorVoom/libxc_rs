//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 637/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk637<F: Float>(t13555: F, t2580: F, t2508: F, t13525: F, t739: F, t738: F, t13195: F, t13201: F, t13226: F, t13537: F, t13539: F, t13544: F, t13547: F, t13550: F, t13554: F, t270: F) -> (F, F, F, F) {
    let t13556 = t2580 * t13555;
    let t13558 = 0.15381052460284448567e-1 * t2508 * t13556;
    let t13559 = t739 * t13525;
    let t13560 = t738 * t13559;
    let t13566 = t13537 + 0.30762104920568897134e-1 * t2508 * t13539 + t13544 - t13547 + t13550 - t13554 + t13558 - 0.76905262301422242837e-2 * t270 * t13560 + 0.2563508743380741428e-2 * t13195 - 0.3845263115071112142e-2 * t13201 - 0.1281754371690370714e-2 * t13226;
    (t13556, t13559, t13560, t13566)
}
