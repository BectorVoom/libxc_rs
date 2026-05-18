//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 404/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk404<F: Float>(t2685: F, t3295: F, t2684: F, t2090: F, t3209: F, t568: F, t3234: F, t836: F, t2087: F, t2098: F, t2103: F, t317: F, t3267: F, t3271: F, t3275: F, t3277: F, t3283: F, t3284: F, t3287: F, t3291: F, t3298: F, t3300: F, t3304: F, t3309: F, t797: F, t813: F, t833: F) -> (F, F, F, F, F, F, F, F) {
    let t3311 = t2685 * t3295;
    let t3312 = t2684 * t3311;
    let t3313 = F::new(0.38342925953920749676e0) * t3312;
    let t3314 = t2090 * t3209;
    let t3315 = t568 * t3314;
    let t3318 = t836 * t3234;
    let t3319 = t568 * t3318;
    let t3322 = F::new(0.35750489951850426669e0) * t3267 * t317 + F::new(0.35750489951850426669e0) * t3271 * t317 + t3275 - F::new(0.10725146985555128001e1) * t3277 * t2098 - t3283 + F::new(0.71500979903700853338e0) * t2103 * t3284 - F::new(0.35750489951850426669e0) * t797 * t3287 - F::new(0.46011511144704899612e1) * t813 * t3291 - t3298 + F::new(0.11502877786176224903e2) * t833 * t3300 - F::new(0.23005755572352449806e1) * t813 * t3304 - F::new(0.7988109573733489516e-2) * t3309 + t3313 - F::new(0.69017266717057349418e1) * t2087 * t3315 + F::new(0.23005755572352449806e1) * t833 * t3319;
    (t3311, t3312, t3313, t3314, t3315, t3318, t3319, t3322)
}
