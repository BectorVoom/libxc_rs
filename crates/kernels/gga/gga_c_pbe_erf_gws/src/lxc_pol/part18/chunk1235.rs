//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1235/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1235<F: Float>(t53178: F, t14452: F, t9270: F, t14759: F, t4414: F, t14633: F, t51666: F, t13888: F, t3306: F, t353: F, t859: F, t14404: F, t19906: F) -> (F, F, F, F, F, F) {
    let t53179 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t53178;
    let t53187 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t9270 * t14452;
    let t53189 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t4414 * t14759;
    let t53198 = t51666 * t14633;
    let t53199 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t53198;
    let t53220 = t859 * t353 * t13888 * t3306;
    let t53224 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t19906 * t14404;
    (t53179, t53187, t53189, t53199, t53220, t53224)
}
