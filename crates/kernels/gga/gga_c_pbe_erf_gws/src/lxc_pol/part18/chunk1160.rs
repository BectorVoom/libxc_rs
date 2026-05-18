//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1160/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1160<F: Float>(t14423: F, t3140: F, t14682: F, t3989: F, t1123: F, t14125: F, t850: F, t833: F, t1178: F, t3307: F, t371: F, t1177: F) -> (F, F, F, F, F, F) {
    let t14683 = t14423 * t3140;
    let t14684 = t14682 * t14683;
    let t14685 = t3989 * t14684;
    let t14688 = t850 * t1123 * t14125;
    let t14689 = t14688 * t833;
    let t14692 = t371 * t1178 * t3307;
    let t14693 = t1177 * t14692;
    (t14684, t14685, t14688, t14689, t14692, t14693)
}
