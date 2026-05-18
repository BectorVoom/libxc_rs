//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1282/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1282<F: Float>(t1178: F, t8713: F, t13783: F, t50998: F, t2299: F, t371: F, t3970: F, t4141: F, t9505: F, t13917: F, t13919: F, t9555: F) -> (F, F, F) {
    let t53860 = t1178 * t8713;
    let t53862 = t50998 * t53860 * t13783;
    let t53865 = t3970 * t2299 * t371;
    let t53867 = t53865 * t4141 * t9505;
    let t53870 = t13917 * t13919 * t9555;
    (t53862, t53867, t53870)
}
