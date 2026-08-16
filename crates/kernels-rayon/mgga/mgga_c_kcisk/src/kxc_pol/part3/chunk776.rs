//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 776/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk776(t11762: f64, t11934: f64, t1908: f64, t1993: f64, t5432: f64, t10358: f64, t10362: f64, t10368: f64, t10373: f64, t10377: f64, t10379: f64, t10384: f64, t10387: f64, t10392: f64, t10397: f64, t10402: f64, t10406: f64, t10410: f64, t10415: f64, t10417: f64, t10421: f64, t2030: f64) -> (f64, f64) {
    let t11935 = t11762 + t11934;
    let t11936 = t1908 * t11935;
    let t11940 = t5432 * t1993;
    let t11956 = -0.34822083333333333333e-2_f64 * t10358 + 0.23214722222222222222e-2_f64 * t10362 + 0.69644166666666666665e-2_f64 * t10368 - 0.579e0_f64 * t11940 * t2030 - 0.58036805555555555555e-2_f64 * t10373 + 0.34822083333333333333e-2_f64 * t10377 + 0.46429444444444444443e-2_f64 * t10379 + 0.69644166666666666665e-2_f64 * t10384 + 0.34822083333333333333e-2_f64 * t10387 - 0.34822083333333333333e-2_f64 * t10392 + 0.23214722222222222222e-2_f64 * t10397 - 0.69644166666666666666e-2_f64 * t10402 + 0.58036805555555555556e-2_f64 * t10406 + 0.30952962962962962963e-2_f64 * t10410 - 0.69644166666666666665e-2_f64 * t10415 - 0.46429444444444444443e-2_f64 * t10417 + 0.69644166666666666666e-2_f64 * t10421;
    (t11936, t11956)
}
