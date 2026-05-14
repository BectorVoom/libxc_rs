//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 948/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk948<F: Float>(t1757: F, t535: F, t6446: F, t1835: F, t209: F, t6447: F, t508: F, t6451: F, t6455: F, t1810: F, t10194: F, t31: F, t4: F, t1808: F, t1809: F, t1820: F, t1848: F, t1849: F, t1850: F, t1859: F, t1865: F, t1867: F, t22075: F, t22095: F, t22100: F, t522: F, t572: F, t573: F, t586: F, t6343: F, t6347: F, t6382: F, t6388: F, t6391: F, t6400: F, t6406: F, t6420: F, t6425: F, t71: F, t84: F) -> (F, F, F, F, F, F) {
    let t22403 = 8.0 * t1757 * t535 * t6446;
    let t22406 = 0.71233333333333333333e-1 * t209 * t1835 * t6447;
    let t22410 = 0.36845452142031360636e2 * t209 * t508 * t6451 * t6455;
    let t22411 = t1810 * t1810;
    let t22417 = 0.11483710345679012345e-1 * t4 * t10194 * t31;
    let t22422 = 0.69263023597503453196e2 * t1865 * t22095 * t586 + 0.12414802127193579148e5 * t6388 * t1820 * t6391 * t1810 - 0.55208163456790123453e-2 * t4 * t10194 * t71 - 0.18989760778855128827e-2 * t4 * t10194 * t84 + 0.21053604230838734656e2 * t1865 * t1850 * t1859 - 0.62336721237753107879e3 * t6406 * t6347 * t1849 - 0.11579285944033451271e4 * t6400 * t6343 * t1810 - 0.46785787179641632568e1 * t1848 * t6420 * t586 - 8.0 * t1809 * t573 * t6382 + 0.61523382126046769581e4 * t6425 * t22100 * t1849 + 0.51947267698127589897e2 * t1865 * t22075 * t1867 + t22403 + t22406 + t22410 - 24.0 * t6400 * t22411 * t572 + t22417 + 0.41096e0 * t522 * t1808 * t1820 * t573;
    (t22403, t22406, t22410, t22411, t22417, t22422)
}
