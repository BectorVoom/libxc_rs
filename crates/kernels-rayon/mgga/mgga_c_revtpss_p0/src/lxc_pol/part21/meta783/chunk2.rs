//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2811/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2811(t231: f64, t2782: f64, t2783: f64, t51625: f64, t14946: f64, t2710: f64, t9285: f64, t40938: f64, t40942: f64, t51617: f64, t51621: f64, t51623: f64, t51628: f64, t51632: f64, t51635: f64, t51637: f64) -> f64 {
    let t51642 = t2782 * t2783 * t51625 * t231;
    let t51646 = t2710 * t14946 * t9285;
    let t51648 = -0.29272321618148349057e-1_f64 * t51617 - 0.58544643236296698113e-1_f64 * t51621 - 0.29272321618148349057e-1_f64 * t51623 - 0.32927245914677557992e-1_f64 * t51628 + 0.11708928647259339623e0_f64 * t51632 + 0.46263278077393568556e-2_f64 * t51635 + 0.19514881078765566037e-2_f64 * t51637 - 0.19514881078765566037e-2_f64 * t40938 + 0.16463622957338778996e-1_f64 * t51642 + 0.9757440539382783019e-2_f64 * t40942 - 0.46263278077393568556e-2_f64 * t51646;
    t51648
}
