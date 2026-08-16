//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2546/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2546<F: Float>(t10423: F, t13995: F, t10413: F, t10422: F, t14221: F, t10949: F, t14025: F, t10883: F, t13969: F, t14106: F, t13559: F, t2970: F, t973: F) -> (F, F, F, F, F) {
    let t49697 = t13995 * t10423;
    let t49702 = t10413 * t10422 * t14221;
    let t49716 = t10949 * t14025;
    let t49721 = t10883 * t13969 * t14106;
    let t49732 = t973 * t2970 * t13559;
    (t49697, t49702, t49716, t49721, t49732)
}
