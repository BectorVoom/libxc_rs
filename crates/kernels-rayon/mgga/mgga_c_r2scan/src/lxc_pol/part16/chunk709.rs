//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 709/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk709(t1938: f64, t5636: f64, t1956: f64, t206: f64, t1923: f64, t689: f64, t1399: f64, t1810: f64, t1814: f64, t1827: f64, t1831: f64, t1838: f64, t390: f64, t5612: f64, t5614: f64, t5629: f64, t5633: f64, t741: f64, t750: f64) -> f64 {
    let t5637 = t5636 * t1938;
    let t5640 = t1956 * t206;
    let t5641 = t689 * t1923;
    let t5642 = t5640 * t5641;
    let t5647 = t5612 - t5614 + 0.32530743900905219526e-1_f64 * t390 * t1831 + 0.28895839882605942646e1_f64 * t390 * t1838 + 0.65061487801810439052e-1_f64 * t390 * t1827 - 0.97592231702715658578e-1_f64 * t390 * t1810 - 0.43374325201206959369e-1_f64 * t1399 * t741 + 0.64212977516902094772e0_f64 * t1399 * t750 - 0.10628661134652298293e3_f64 * t390 * t5629 - 0.16522625736956710527e1_f64 * t390 * t5633 - 0.33045251473913421054e1_f64 * t390 * t5637 + 0.99135754421740263165e1_f64 * t390 * t5642 - 0.48159733137676571079e0_f64 * t390 * t1814;
    t5647
}
