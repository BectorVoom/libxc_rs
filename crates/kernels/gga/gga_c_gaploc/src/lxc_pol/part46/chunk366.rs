//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 366/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk366<F: Float>(t2487: F, t3196: F, t1565: F, t3085: F, t568: F, t3116: F, t600: F, t1532: F, t1562: F, t1572: F, t193: F, t3149: F, t3153: F, t3157: F, t3159: F, t3165: F, t3166: F, t3169: F, t3173: F, t3180: F, t3182: F, t3186: F, t3194: F, t557: F, t574: F, t597: F) -> (F, F, F, F, F, F) {
    let t3197 = t2487 * t3196;
    let t3198 = 0.38342925953920749676e0 * t3197;
    let t3199 = t1565 * t3085;
    let t3200 = t568 * t3199;
    let t3203 = t600 * t3116;
    let t3204 = t568 * t3203;
    let t3207 = 0.35750489951850426669e0 * t3149 * t193 + 0.35750489951850426669e0 * t3153 * t193 + t3157 - 0.10725146985555128001e1 * t3159 * t1532 - t3165 + 0.71500979903700853338e0 * t1572 * t3166 - 0.35750489951850426669e0 * t557 * t3169 - 0.46011511144704899612e1 * t574 * t3173 - t3180 + 0.11502877786176224903e2 * t597 * t3182 - 0.23005755572352449806e1 * t574 * t3186 - 0.7988109573733489516e-2 * t3194 + t3198 - 0.69017266717057349418e1 * t1562 * t3200 + 0.23005755572352449806e1 * t597 * t3204;
    (t3198, t3199, t3200, t3203, t3204, t3207)
}
