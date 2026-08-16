//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1396/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1396(t4227: f64, t6126: f64, t11737: f64, t1206: f64, t12248: f64, t14185: f64, t14882: f64, t15021: f64, t15558: f64, t2408: f64, t29751: f64, t3066: f64, t3068: f64, t3207: f64, t35566: f64, t55831: f64, t55833: f64, t55841: f64, t57545: f64, t57551: f64, t57555: f64, t57570: f64, t57574: f64, t57578: f64, t9283: f64) -> f64 {
    let t58854 = t6126 * t4227;
    let t58869 = t55831 + t55833 - t57545 / 24.0_f64 - t2408 * t29751 * t15558 / 12.0_f64 + t57551 / 24.0_f64 + t57555 / 768.0_f64 - t3066 * t35566 * t14882 / 8.0_f64 + t55841 - t57570 / 256.0_f64 - t2408 * t35566 * t15021 / 12.0_f64 - t3066 * t9283 * t58854 * t3068 / 8.0_f64 + t57574 / 768.0_f64 + t57578 / 48.0_f64 - t2408 * t9283 * t14185 * t12248 / 24.0_f64 - t3207 * t9283 * t1206 * t11737 / 16.0_f64;
    t58869
}
