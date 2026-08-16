//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2814/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2814(t2723: f64, t2782: f64, t4503: f64, t76131: f64, t1558: f64, t6041: f64, t231: f64, t2783: f64, t4500: f64, t62967: f64, t10661: f64, t14972: f64, t23172: f64, t4366: f64, t4504: f64, t51299: f64, t6017: f64, t62606: f64, t62609: f64, t76117: f64, t76125: f64, t76127: f64, t820: f64) -> (f64, f64) {
    let t76134 = t2782 * t4503 * t76131 * t2723;
    let t76136 = t6041 * t1558;
    let t76139 = t2782 * t2783 * t76136 * t231;
    let t76144 = t62967 * t4500;
    let t76147 = -0.54878743191129263322e-2_f64 * t76117 - 0.19756347548806534796e1_f64 * t820 * t14972 * t6017 - 0.32927245914677557992e-1_f64 * t62606 + 0.58544643236296698112e-1_f64 * t76125 + 0.13170898365871023197e1_f64 * t4504 * t76127 * t4366 - t51299 - 0.32927245914677557992e-1_f64 * t76134 + 0.16463622957338778997e-1_f64 * t76139 + 0.39512695097613069591e1_f64 * t820 * t10661 * t23172 - 0.29272321618148349057e-1_f64 * t76144 - 0.58544643236296698114e-1_f64 * t62609;
    (t76136, t76147)
}
