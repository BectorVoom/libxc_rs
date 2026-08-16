//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 441/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk441(t221: f64, t2965: f64, t339: f64, t964: f64, t995: f64, t1000: f64, t1020: f64, t1025: f64, t1046: f64, t2955: f64, t2960: f64, t3109: f64, t3114: f64, t3117: f64, t3123: f64, t3130: f64, t3134: f64, t3140: f64, t3143: f64, t3148: f64, t3153: f64, t3156: f64, t350: f64, t973: f64) -> f64 {
    let t3158 = t221 * t2965;
    let t3160 = t339 * t3158 / 432.0_f64;
    let t3163 = t964 * t995;
    let t3165 = -t3109 * t1025 / 288.0_f64 + t3114 * t1025 / 1536.0_f64 + t3117 * t1046 / 2304.0_f64 + t1020 * t3123 / 3072.0_f64 + t3130 * t3134 / 1536.0_f64 - t2960 * t1000 / 54.0_f64 + t3140 / 432.0_f64 + t973 * t3143 / 288.0_f64 + t973 * t3148 / 216.0_f64 - t973 * t3153 / 144.0_f64 + t3156 / 2304.0_f64 - t3160 + 11.0_f64 / 108.0_f64 * t2955 * t350 - t3163 / 54.0_f64;
    t3165
}
