//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 980/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk980<F: Float>(t17926: F, t285: F, t1506: F, t4570: F, t3119: F, t12621: F, t123: F, t3108: F, t15236: F, t1111: F, t11937: F, t12490: F, t1503: F, t15660: F, t15690: F, t15694: F, t15697: F, t17898: F, t17904: F, t17908: F, t17922: F, t3103: F, t3116: F, t431: F, t4310: F, t5286: F, t5290: F, t5319: F, t5330: F, t8973: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t17927 = sigma2 * t17926;
    let t17928 = t17927 * t285;
    let t17935 = t4570 * t1506;
    let t17936 = t17935 * t3119;
    let t17937 = t12621 * t17936;
    let t17940 = t1506 * t123;
    let t17941 = t3108 * t17940;
    let t17942 = t15236 * t17941;
    let t17945 = -t1111 * t17898 / F::new(36.0) + t4310 * t5319 / F::new(18.0) + t1111 * t17904 / F::new(288.0) + F::new(7.0) / F::new(648.0) * t1111 * t17908 + F::new(11.0) / F::new(108.0) * t15690 * t1503 - t4310 * t5286 / F::new(36.0) - t4310 * t5290 / F::new(27.0) - F::cast_from(0.1465164556873572827e3_f64) * t11937 * t5330 + F::cast_from(0.18314556960919660338e2_f64) * t8973 * t17922 - F::new(77.0) / F::new(162.0) * t17928 * t431 + F::cast_from(0.18352229811776266582e0_f64) * t15660 - F::cast_from(0.12073835402484385909e-2_f64) * t12490 - t15694 / F::new(144.0) + t15697 / F::new(216.0) + F::cast_from(0.11833438829693848058e0_f64) * t3116 * t17937 + F::cast_from(0.27471835441379490507e2_f64) * t3103 * t17942;
    (t17927, t17928, t17936, t17937, t17940, t17941, t17942, t17945)
}
