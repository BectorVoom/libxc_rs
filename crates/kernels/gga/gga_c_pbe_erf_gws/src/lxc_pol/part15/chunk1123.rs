//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1123/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1123<F: Float>(t13783: F, t50998: F, t53860: F, t2299: F, t371: F, t3970: F, t4141: F, t9505: F, t13917: F, t13919: F, t9555: F, t14404: F, t19704: F, t51756: F, t51758: F, t51769: F, t51771: F, t51781: F, t51788: F, t53843: F, t53846: F, t53848: F, t53852: F, t53856: F) -> (F,) {
    let t53862 = t50998 * t53860 * t13783;
    let t53865 = t3970 * t2299 * t371;
    let t53867 = t53865 * t4141 * t9505;
    let t53870 = t13917 * t13919 * t9555;
    let t53872 = 7.0 / 144.0 * t51756 - 7.0 / 72.0 * t51758 + 7.0 / 48.0 * t51769 - t53843 / 8.0 - 7.0 / 2304.0 * t51771 + t53846 / 24.0 + t53848 / 48.0 + 7.0 / 144.0 * t51781 + 7.0 / 288.0 * t51788 - 35.0 / 432.0 * t53852 + t53856 / 384.0 + t19704 * t14404 / 48.0 + t53862 / 192.0 + 5.0 / 192.0 * t53867 - t53870 / 1536.0;
    (t53872,)
}
