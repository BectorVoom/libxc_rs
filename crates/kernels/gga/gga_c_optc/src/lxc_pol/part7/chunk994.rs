//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 994/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk994<F: Float>(t21931: F, t1948: F, t616: F, t1880: F, t1953: F, t201: F, t21907: F, t21911: F, t21913: F, t21915: F, t21920: F, t21929: F, t3316: F, t3318: F, t3539: F, t6672: F, t7159: F, t755: F, t9361: F, t95: F, t9548: F) -> (F, F, F) {
    let t21932 = F::new(960.0) * t21931;
    let t21933 = t1948 * t616;
    let t21937 = -t21907 + F::cast_from(0.93041573165652349788e-1_f64) * t95 * t9361 * t1948 + F::new(6.0) * t21911 + F::new(6.0) * t21913 + F::new(3.0) * t3316 * t3318 * t21915 * t1953 - F::new(14.0) * t21920 + F::new(2.0) * t3316 * t3318 * t6672 * t755 * t201 + F::new(6.0) * t9548 * t7159 + F::new(6.0) * t21929 + t21932 + F::cast_from(0.18608314633130469958e0_f64) * t3539 * t1880 * t21933;
    (t21932, t21933, t21937)
}
