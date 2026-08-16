//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 980/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk980(t17926: f64, t285: f64, t1506: f64, t4570: f64, t3119: f64, t12621: f64, t123: f64, t3108: f64, t15236: f64, t1111: f64, t11937: f64, t12490: f64, t1503: f64, t15660: f64, t15690: f64, t15694: f64, t15697: f64, t17898: f64, t17904: f64, t17908: f64, t17922: f64, t3103: f64, t3116: f64, t431: f64, t4310: f64, t5286: f64, t5290: f64, t5319: f64, t5330: f64, t8973: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t17927 = sigma2 * t17926;
    let t17928 = t17927 * t285;
    let t17935 = t4570 * t1506;
    let t17936 = t17935 * t3119;
    let t17937 = t12621 * t17936;
    let t17940 = t1506 * t123;
    let t17941 = t3108 * t17940;
    let t17942 = t15236 * t17941;
    let t17945 = -t1111 * t17898 / 36.0_f64 + t4310 * t5319 / 18.0_f64 + t1111 * t17904 / 288.0_f64 + 7.0_f64 / 648.0_f64 * t1111 * t17908 + 11.0_f64 / 108.0_f64 * t15690 * t1503 - t4310 * t5286 / 36.0_f64 - t4310 * t5290 / 27.0_f64 - 0.1465164556873572827e3_f64 * t11937 * t5330 + 0.18314556960919660338e2_f64 * t8973 * t17922 - 77.0_f64 / 162.0_f64 * t17928 * t431 + 0.18352229811776266582e0_f64 * t15660 - 0.12073835402484385909e-2_f64 * t12490 - t15694 / 144.0_f64 + t15697 / 216.0_f64 + 0.11833438829693848058e0_f64 * t3116 * t17937 + 0.27471835441379490507e2_f64 * t3103 * t17942;
    (t17927, t17928, t17936, t17937, t17940, t17941, t17942, t17945)
}
