//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 385/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk385<F: Float>(t3203: F, t568: F, t1532: F, t1562: F, t1572: F, t193: F, t3149: F, t3153: F, t3157: F, t3159: F, t3165: F, t3166: F, t3169: F, t3173: F, t3180: F, t3182: F, t3186: F, t3194: F, t3198: F, t3200: F, t557: F, t574: F, t597: F) -> (F, F) {
    let t3204 = t568 * t3203;
    let t3207 = F::cast_from(0.35750489951850426669e0_f64) * t3149 * t193 + F::cast_from(0.35750489951850426669e0_f64) * t3153 * t193 + t3157 - F::cast_from(0.10725146985555128001e1_f64) * t3159 * t1532 - t3165 + F::cast_from(0.71500979903700853338e0_f64) * t1572 * t3166 - F::cast_from(0.35750489951850426669e0_f64) * t557 * t3169 - F::cast_from(0.46011511144704899612e1_f64) * t574 * t3173 - t3180 + F::cast_from(0.11502877786176224903e2_f64) * t597 * t3182 - F::cast_from(0.23005755572352449806e1_f64) * t574 * t3186 - F::cast_from(0.7988109573733489516e-2_f64) * t3194 + t3198 - F::cast_from(0.69017266717057349418e1_f64) * t1562 * t3200 + F::cast_from(0.23005755572352449806e1_f64) * t597 * t3204;
    (t3204, t3207)
}
