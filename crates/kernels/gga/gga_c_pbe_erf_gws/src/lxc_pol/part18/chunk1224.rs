//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1224/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1224<F: Float>(t51869: F, t13987: F, t894: F, t3958: F, t6659: F, t26730: F, t353: F, t859: F, t332: F, t6158: F, t4408: F, t1195: F, t6729: F) -> (F, F, F, F, F, F, F) {
    let t51870 = F::cast_from(595.0_f64) / F::cast_from(10368.0_f64) * t51869;
    let t51877 = t13987 * t894;
    let t51898 = t3958 * t6659;
    let t51913 = t859 * t353 * t26730;
    let t51916 = t6158 * t332;
    let t51922 = t4408 * t332;
    let t51957 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t6729 * t1195;
    (t51870, t51877, t51898, t51913, t51916, t51922, t51957)
}
