//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 472/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk472(t2265: f64, t2287: f64, t2289: f64, t2290: f64, t153: f64, t474: f64, t865: f64, t1197: f64, t1199: f64, t1202: f64, t1206: f64, t1209: f64, t1213: f64, t1215: f64, t1540: f64, t1542: f64, t156: f64, t168: f64, t1891: f64, t1905: f64, t2240: f64, t2244: f64, t2249: f64, t242: f64, t245: f64) -> (f64, f64, f64) {
    let t2292 = t2265 + t2287 + t2289 + t2290;
    let t2298 = t153 * t474 * t865;
    let t2303 = -t1197 + 0.0837628205355044_f64 * t1199 + t1202 + 0.0837628205355044_f64 * t2240 - 0.0837628205355044_f64 * t1905 * t242 - 0.0837628205355044_f64 * t2244 - 0.0837628205355044_f64 * t1206 - t1209 - t1213 + 0.019897291109174608_f64 * t1215 + 0.019897291109174608_f64 * t2249 - 0.011938374665504766_f64 * t168 * t245 * t2292 + t1540 - 0.5694518669548363_f64 * t1542 - 0.5694518669548363_f64 * t2298 + 0.42708890021612717_f64 * t153 * t156 * t1891;
    (t2292, t2298, t2303)
}
