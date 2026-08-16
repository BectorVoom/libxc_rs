//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 612/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk612<F: Float>(t1860: F, t695: F, t1060: F, t1919: F, t1849: F, t702: F, t3290: F, t1920: F, t3293: F, t5172: F, t673: F, t140: F, t1470: F, t1883: F, t1888: F, t1909: F, t4625: F, t4631: F, t4653: F, t4659: F, t4685: F, t479: F, t5222: F, t5231: F, t5242: F, t5243: F, t5246: F, t5251: F, t709: F, t725: F) -> (F, F, F, F, F, F) {
    let t5254 = t1860 * t695;
    let t5256 = t1919 * t5254 * t1060;
    let t5259 = t702 * t1849;
    let t5261 = t1919 * t5259 * t3290;
    let t5265 = t1919 * t1920 * t3293;
    let t5268 = t673 * t5172;
    let t5272 = F::cast_from(0.619125e-2_f64) * t5222 * t709 + F::cast_from(0.1857375e-1_f64) * t1909 * t1883 - F::cast_from(0.123825e-1_f64) * t1909 * t1888 + F::cast_from(0.46434375e-2_f64) * t725 * t4625 - F::cast_from(0.1857375e-1_f64) * t5231 * t4631 + F::cast_from(0.9286875e-2_f64) * t725 * t4653 + F::cast_from(0.123825e-1_f64) * t725 * t4659 - F::cast_from(0.619125e-2_f64) * t725 * t4685 + t5242 - F::cast_from(0.35374814814814814814e-1_f64) * t5243 - F::cast_from(0.53062222222222222222e-1_f64) * t5246 - F::cast_from(0.44218518518518518518e-1_f64) * t1470 * t5251 - F::cast_from(0.53062222222222222222e-1_f64) * t1470 * t5256 + F::cast_from(0.53062222222222222222e-1_f64) * t1470 * t5261 - F::cast_from(0.26531111111111111111e-1_f64) * t1470 * t5265 - F::cast_from(0.39796666666666666666e-1_f64) * t140 * t479 * t5268;
    (t5254, t5256, t5261, t5265, t5268, t5272)
}
