//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1640/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1640(t1012: f64, t1222: f64, t13095: f64, t17261: f64, t3699: f64, t39443: f64, t39449: f64, t43847: f64, t43852: f64, t44898: f64, t44902: f64, t44906: f64, t44912: f64, t44917: f64, t44919: f64, t44925: f64, t44928: f64, t44931: f64, t44938: f64, t5308: f64, t5312: f64) -> f64 {
    let t44942 = -0.16937883700965822014e-2_f64 * t44898 + 0.19055119163586549765e-3_f64 * t44902 + 0.3811023832717309953e-3_f64 * t44906 + t1222 * t5312 * t43852 / 6.0_f64 - t44912 / 36.0_f64 - t1222 * t5308 * t43847 / 36.0_f64 + 0.38110238327173099531e-3_f64 * t44917 - t1222 * t1012 * t44919 * t39443 / 12.0_f64 + t44925 / 216.0_f64 - t44928 / 216.0_f64 - 5.0_f64 / 972.0_f64 * t44931 + t1222 * t1012 * t3699 * t39449 / 72.0_f64 - 0.34299214494455789578e-2_f64 * t44938 + 0.51448821741683684368e-2_f64 * t17261 * t13095;
    t44942
}
