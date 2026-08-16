//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1323/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1323(t11511: f64, t4023: f64, t11878: f64, t11883: f64, t14028: f64, t3749: f64, t11924: f64, t51334: f64, t854: f64, t14024: f64, t3788: f64, t11651: f64, t338: f64, t54090: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56929 = t11511 * t4023;
    let t56931 = t11878 * t4023;
    let t56933 = t11883 * t4023;
    let t56935 = t14028 * t3749;
    let t56937 = t51334 * t11924;
    let t56938 = t854 * t56937;
    let t56940 = t3788 * t14024;
    let t56943 = t54090 * t338 * t11651;
    (t56929, t56931, t56933, t56935, t56938, t56940, t56943)
}
