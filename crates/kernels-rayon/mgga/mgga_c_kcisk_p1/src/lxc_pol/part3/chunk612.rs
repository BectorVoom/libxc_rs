//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 612/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk612(t1860: f64, t695: f64, t1060: f64, t1919: f64, t1849: f64, t702: f64, t3290: f64, t1920: f64, t3293: f64, t5172: f64, t673: f64, t140: f64, t1470: f64, t1883: f64, t1888: f64, t1909: f64, t4625: f64, t4631: f64, t4653: f64, t4659: f64, t4685: f64, t479: f64, t5222: f64, t5231: f64, t5242: f64, t5243: f64, t5246: f64, t5251: f64, t709: f64, t725: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5254 = t1860 * t695;
    let t5256 = t1919 * t5254 * t1060;
    let t5259 = t702 * t1849;
    let t5261 = t1919 * t5259 * t3290;
    let t5265 = t1919 * t1920 * t3293;
    let t5268 = t673 * t5172;
    let t5272 = 0.619125e-2_f64 * t5222 * t709 + 0.1857375e-1_f64 * t1909 * t1883 - 0.123825e-1_f64 * t1909 * t1888 + 0.46434375e-2_f64 * t725 * t4625 - 0.1857375e-1_f64 * t5231 * t4631 + 0.9286875e-2_f64 * t725 * t4653 + 0.123825e-1_f64 * t725 * t4659 - 0.619125e-2_f64 * t725 * t4685 + t5242 - 0.35374814814814814814e-1_f64 * t5243 - 0.53062222222222222222e-1_f64 * t5246 - 0.44218518518518518518e-1_f64 * t1470 * t5251 - 0.53062222222222222222e-1_f64 * t1470 * t5256 + 0.53062222222222222222e-1_f64 * t1470 * t5261 - 0.26531111111111111111e-1_f64 * t1470 * t5265 - 0.39796666666666666666e-1_f64 * t140 * t479 * t5268;
    (t5254, t5256, t5261, t5265, t5268, t5272)
}
