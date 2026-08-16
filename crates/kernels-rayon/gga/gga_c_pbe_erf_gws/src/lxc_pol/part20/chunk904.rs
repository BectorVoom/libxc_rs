//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 904/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk904(t8144: f64, t5796: f64, t5816: f64, t3668: f64, t481: f64, t8197: f64, t10063: f64, t120: f64, t102: f64, t506: f64, t3644: f64, t5825: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10090 = 0.64956111111111111111e0_f64 * t8144;
    let t10094 = 0.32478055555555555555e0_f64 * t5796;
    let t10096 = 0.64956111111111111111e0_f64 * t5816;
    let t10097 = t3668 * t481;
    let t10102 = 0.12991222222222222222e1_f64 * t8197;
    let t10104 = t120 * t10063;
    let t10106 = 0.2923025e1_f64 * t102 * t10104;
    let t10107 = t506 * t10063;
    let t10110 = t5825 * t3644;
    (t10090, t10094, t10096, t10097, t10102, t10106, t10107, t10110)
}
