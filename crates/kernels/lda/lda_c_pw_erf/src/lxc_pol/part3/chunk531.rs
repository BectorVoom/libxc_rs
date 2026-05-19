//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 531/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk531<F: Float>(t2265: F, t2287: F, t2289: F, t2290: F, t153: F, t474: F, t865: F, t1197: F, t1199: F, t1202: F, t1206: F, t1209: F, t1213: F, t1215: F, t1540: F, t1542: F, t156: F, t168: F, t1891: F, t1905: F, t2240: F, t2244: F, t2249: F, t242: F, t245: F) -> (F, F) {
    let t2292 = t2265 + t2287 + t2289 + t2290;
    let t2298 = t153 * t474 * t865;
    let t2303 = -t1197 + F::cast_from(0.0837628205355044_f64) * t1199 + t1202 + F::cast_from(0.0837628205355044_f64) * t2240 - F::cast_from(0.0837628205355044_f64) * t1905 * t242 - F::cast_from(0.0837628205355044_f64) * t2244 - F::cast_from(0.0837628205355044_f64) * t1206 - t1209 - t1213 + F::cast_from(0.019897291109174608_f64) * t1215 + F::cast_from(0.019897291109174608_f64) * t2249 - F::cast_from(0.011938374665504766_f64) * t168 * t245 * t2292 + t1540 - F::cast_from(0.5694518669548363_f64) * t1542 - F::cast_from(0.5694518669548363_f64) * t2298 + F::cast_from(0.42708890021612717_f64) * t153 * t156 * t1891;
    (t2292, t2303)
}
