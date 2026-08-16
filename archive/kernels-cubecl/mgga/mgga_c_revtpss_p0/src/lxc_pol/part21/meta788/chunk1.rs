//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2837/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2837<F: Float>(t15162: F, t698: F, t141: F, t2908: F, t51873: F, t15165: F, t51847: F, t930: F, t41246: F, t41267: F, t41275: F, t51921: F, t51923: F, t51927: F, t51932: F, t51935: F) -> (F, F, F, F, F) {
    let t51937 = t698 * t15162;
    let t51940 = t141 * t2908 * t51873;
    let t51942 = t698 * t15165;
    let t51945 = t141 * t930 * t51847;
    let t51949 = F::cast_from(0.55190000000000000001e-1_f64) * t51921 + F::cast_from(0.73586666666666666668e-1_f64) * t51923 - F::cast_from(0.82785e-1_f64) * t51927 - F::cast_from(0.11038e0_f64) * t51932 - F::cast_from(0.27595e-1_f64) * t51935 - F::cast_from(0.33114000000000000001e0_f64) * t51937 - F::cast_from(0.99342e0_f64) * t51940 + F::cast_from(0.99342e0_f64) * t51942 + F::cast_from(0.198684e1_f64) * t51945 + t41246 - F::cast_from(0.33114e0_f64) * t41267 + F::cast_from(0.33114e0_f64) * t41275;
    (t51937, t51940, t51942, t51945, t51949)
}
