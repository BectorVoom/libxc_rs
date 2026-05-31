//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 776/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk776<F: Float>(t11762: F, t11934: F, t1908: F, t1993: F, t5432: F, t10358: F, t10362: F, t10368: F, t10373: F, t10377: F, t10379: F, t10384: F, t10387: F, t10392: F, t10397: F, t10402: F, t10406: F, t10410: F, t10415: F, t10417: F, t10421: F, t2030: F) -> (F, F) {
    let t11935 = t11762 + t11934;
    let t11936 = t1908 * t11935;
    let t11940 = t5432 * t1993;
    let t11956 = -F::cast_from(0.34822083333333333333e-2_f64) * t10358 + F::cast_from(0.23214722222222222222e-2_f64) * t10362 + F::cast_from(0.69644166666666666665e-2_f64) * t10368 - F::cast_from(0.579e0_f64) * t11940 * t2030 - F::cast_from(0.58036805555555555555e-2_f64) * t10373 + F::cast_from(0.34822083333333333333e-2_f64) * t10377 + F::cast_from(0.46429444444444444443e-2_f64) * t10379 + F::cast_from(0.69644166666666666665e-2_f64) * t10384 + F::cast_from(0.34822083333333333333e-2_f64) * t10387 - F::cast_from(0.34822083333333333333e-2_f64) * t10392 + F::cast_from(0.23214722222222222222e-2_f64) * t10397 - F::cast_from(0.69644166666666666666e-2_f64) * t10402 + F::cast_from(0.58036805555555555556e-2_f64) * t10406 + F::cast_from(0.30952962962962962963e-2_f64) * t10410 - F::cast_from(0.69644166666666666665e-2_f64) * t10415 - F::cast_from(0.46429444444444444443e-2_f64) * t10417 + F::cast_from(0.69644166666666666666e-2_f64) * t10421;
    (t11936, t11956)
}
