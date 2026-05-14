//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1343/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1343<F: Float>(t153: F, t474: F, t6080: F, t1210: F, t168: F, t2581: F, t11002: F, t11006: F, t11007: F, t11010: F, t11012: F, t14921: F, t14925: F, t156: F, t17050: F, t17896: F, t19005: F, t19105: F, t19106: F, t19108: F, t19109: F, t19112: F, t19115: F, t19117: F, t19118: F, t19122: F, t19127: F, t19129: F, t19130: F, t19133: F, t19180: F, t19183: F, t19184: F, t19189: F, t19190: F, t19192: F, t19193: F, t19196: F, t19198: F, t19200: F, t19202: F, t19206: F, t19214: F, t19232: F, t19238: F, t19239: F, t19241: F, t19242: F, t19248: F, t19253: F, t19255: F, t19258: F, t19261: F, t19262: F, t19264: F, t19265: F, t19269: F, t19270: F, t19272: F, t19277: F, t19280: F, t19282: F, t19283: F, t19288: F, t19289: F, t19294: F, t19295: F, t19298: F, t19299: F, t19301: F, t19302: F, t19306: F, t19316: F, t19322: F, t19323: F, t19326: F, t19329: F, t19331: F, t19333: F, t245: F) -> (F,) {
    let t19344 = t153 * t474 * t6080;
    let t19347 = t168 * t1210 * t2581;
    let t19355 = 1.328721022894618 * t11002 + t11006 - 0.011938374665504766 * t168 * t245 * (t19133 + t19112 + t19322 + t19183 + t19184 + t19129 + t19130 + t19299 + t19301 + t19302 + t19196 + t19306 + t19323 + t19326 + t19329 + t19253 + t19255 + t19294 + t19272 + t19108 + t19109 + t19189 + t19288 + t19289 + t19248 + t19283 + t19118 + t17896 + t19242 + t19269 + t19270 + t19214 + t19277 + t19295 + t19298 + t19206 + t19316 + t19190 + t19192 + t19193 + t19115 + t19117 + t19127 + t19280 + t19282 + t19202 + t19180 + t19265 + t19258 + t19238 + t19239 + t19241 + t19105 + t19106 + t17050 + t19232 + t19198 + t19200 + t19331 + t19333 + t19261 + t19262 + t19264 + t19122) - 1.1389037339096726 * t19344 - 0.053059442957798957 * t19347 + 0.42708890021612717 * t153 * t156 * t19005 - 1.1389037339096726 * t14921 + 1.0051538464260528 * t11007 + t11010 + 0.3891025816905257 * t14925 - t11012;
    (t19355,)
}
