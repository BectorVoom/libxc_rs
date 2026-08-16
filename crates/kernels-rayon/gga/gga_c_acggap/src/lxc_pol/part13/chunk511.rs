//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 511/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk511(t283: f64, t2965: f64, t2868: f64, t88: f64, t228: f64, t35: f64, t223: f64, t595: f64, t1964: f64, t36: f64, t265: f64, t272: f64, t2787: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2966 = t2965 * t283;
    let t2968 = t2868 * t88;
    let t2969 = 24.0_f64 * t2968;
    let t2970 = t35 * t228;
    let t2971 = t2970 * t88;
    let t2974 = t223 * t595;
    let t2975 = t2974 * t88;
    let t2977 = t36 * t1964;
    let t2978 = t2977 * t88;
    let t2979 = 120.0_f64 * t2978;
    let t2981 = t265 * t2787 * t272;
    (t2966, t2969, t2971, t2975, t2979, t2981)
}
