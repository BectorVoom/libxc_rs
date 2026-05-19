//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1029/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1029<F: Float>(t10194: F, t1808: F, t1809: F, t1810: F, t1820: F, t1848: F, t1849: F, t1850: F, t1859: F, t1865: F, t1867: F, t22075: F, t22095: F, t22100: F, t22403: F, t22406: F, t22410: F, t22411: F, t22417: F, t4: F, t522: F, t572: F, t573: F, t586: F, t6343: F, t6347: F, t6382: F, t6388: F, t6391: F, t6400: F, t6406: F, t6420: F, t6425: F, t71: F, t84: F) -> F {
    let t22422 = F::cast_from(0.69263023597503453196e2_f64) * t1865 * t22095 * t586 + F::cast_from(0.12414802127193579148e5_f64) * t6388 * t1820 * t6391 * t1810 - F::cast_from(0.55208163456790123453e-2_f64) * t4 * t10194 * t71 - F::cast_from(0.18989760778855128827e-2_f64) * t4 * t10194 * t84 + F::cast_from(0.21053604230838734656e2_f64) * t1865 * t1850 * t1859 - F::cast_from(0.62336721237753107879e3_f64) * t6406 * t6347 * t1849 - F::cast_from(0.11579285944033451271e4_f64) * t6400 * t6343 * t1810 - F::cast_from(0.46785787179641632568e1_f64) * t1848 * t6420 * t586 - F::new(8.0) * t1809 * t573 * t6382 + F::cast_from(0.61523382126046769581e4_f64) * t6425 * t22100 * t1849 + F::cast_from(0.51947267698127589897e2_f64) * t1865 * t22075 * t1867 + t22403 + t22406 + t22410 - F::new(24.0) * t6400 * t22411 * t572 + t22417 + F::new(0.41096e0) * t522 * t1808 * t1820 * t573;
    t22422
}
