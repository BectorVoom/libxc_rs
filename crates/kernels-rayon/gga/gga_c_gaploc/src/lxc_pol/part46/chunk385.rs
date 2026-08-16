//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 385/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk385(t3203: f64, t568: f64, t1532: f64, t1562: f64, t1572: f64, t193: f64, t3149: f64, t3153: f64, t3157: f64, t3159: f64, t3165: f64, t3166: f64, t3169: f64, t3173: f64, t3180: f64, t3182: f64, t3186: f64, t3194: f64, t3198: f64, t3200: f64, t557: f64, t574: f64, t597: f64) -> (f64, f64) {
    let t3204 = t568 * t3203;
    let t3207 = 0.35750489951850426669e0_f64 * t3149 * t193 + 0.35750489951850426669e0_f64 * t3153 * t193 + t3157 - 0.10725146985555128001e1_f64 * t3159 * t1532 - t3165 + 0.71500979903700853338e0_f64 * t1572 * t3166 - 0.35750489951850426669e0_f64 * t557 * t3169 - 0.46011511144704899612e1_f64 * t574 * t3173 - t3180 + 0.11502877786176224903e2_f64 * t597 * t3182 - 0.23005755572352449806e1_f64 * t574 * t3186 - 0.7988109573733489516e-2_f64 * t3194 + t3198 - 0.69017266717057349418e1_f64 * t1562 * t3200 + 0.23005755572352449806e1_f64 * t597 * t3204;
    (t3204, t3207)
}
