//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1029/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1029(t10194: f64, t1808: f64, t1809: f64, t1810: f64, t1820: f64, t1848: f64, t1849: f64, t1850: f64, t1859: f64, t1865: f64, t1867: f64, t22075: f64, t22095: f64, t22100: f64, t22403: f64, t22406: f64, t22410: f64, t22411: f64, t22417: f64, t4: f64, t522: f64, t572: f64, t573: f64, t586: f64, t6343: f64, t6347: f64, t6382: f64, t6388: f64, t6391: f64, t6400: f64, t6406: f64, t6420: f64, t6425: f64, t71: f64, t84: f64) -> f64 {
    let t22422 = 0.69263023597503453196e2_f64 * t1865 * t22095 * t586 + 0.12414802127193579148e5_f64 * t6388 * t1820 * t6391 * t1810 - 0.55208163456790123453e-2_f64 * t4 * t10194 * t71 - 0.18989760778855128827e-2_f64 * t4 * t10194 * t84 + 0.21053604230838734656e2_f64 * t1865 * t1850 * t1859 - 0.62336721237753107879e3_f64 * t6406 * t6347 * t1849 - 0.11579285944033451271e4_f64 * t6400 * t6343 * t1810 - 0.46785787179641632568e1_f64 * t1848 * t6420 * t586 - 8.0_f64 * t1809 * t573 * t6382 + 0.61523382126046769581e4_f64 * t6425 * t22100 * t1849 + 0.51947267698127589897e2_f64 * t1865 * t22075 * t1867 + t22403 + t22406 + t22410 - 24.0_f64 * t6400 * t22411 * t572 + t22417 + 0.41096e0_f64 * t522 * t1808 * t1820 * t573;
    t22422
}
