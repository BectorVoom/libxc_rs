//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 401/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk401(t2685: f64, t3295: f64, t2684: f64, t2090: f64, t3209: f64, t568: f64, t3234: f64, t836: f64, t2087: f64, t2098: f64, t2103: f64, t317: f64, t3267: f64, t3271: f64, t3275: f64, t3277: f64, t3283: f64, t3284: f64, t3287: f64, t3291: f64, t3298: f64, t3300: f64, t3304: f64, t3309: f64, t797: f64, t813: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3311 = t2685 * t3295;
    let t3312 = t2684 * t3311;
    let t3313 = 0.38342925953920749676e0_f64 * t3312;
    let t3314 = t2090 * t3209;
    let t3315 = t568 * t3314;
    let t3318 = t836 * t3234;
    let t3319 = t568 * t3318;
    let t3322 = 0.35750489951850426669e0_f64 * t3267 * t317 + 0.35750489951850426669e0_f64 * t3271 * t317 + t3275 - 0.10725146985555128001e1_f64 * t3277 * t2098 - t3283 + 0.71500979903700853338e0_f64 * t2103 * t3284 - 0.35750489951850426669e0_f64 * t797 * t3287 - 0.46011511144704899612e1_f64 * t813 * t3291 - t3298 + 0.11502877786176224903e2_f64 * t833 * t3300 - 0.23005755572352449806e1_f64 * t813 * t3304 - 0.7988109573733489516e-2_f64 * t3309 + t3313 - 0.69017266717057349418e1_f64 * t2087 * t3315 + 0.23005755572352449806e1_f64 * t833 * t3319;
    (t3311, t3313, t3314, t3315, t3318, t3319, t3322)
}
