//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 403/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk403(t2268: f64, t3347: f64, t2778: f64, t883: f64, t2325: f64, t882: f64, t2787: f64, t874: f64) -> (f64, f64, f64, f64) {
    let t3349 = 0.85365019907028448797e-1_f64 * t2268 * t3347;
    let t3350 = t883 * t2778;
    let t3351 = t2325 * t3350;
    let t3352 = t882 * t3351;
    let t3353 = 0.11856252764865062333e-2_f64 * t3352;
    let t3354 = t2787 * t874;
    (t3349, t3351, t3353, t3354)
}
