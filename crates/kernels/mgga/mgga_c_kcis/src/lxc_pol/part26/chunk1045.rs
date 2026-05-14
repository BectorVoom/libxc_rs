//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1045/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1045<F: Float>(t29269: F, t7923: F, t1394: F, t5662: F, t6281: F, t4153: F, t27387: F, t7100: F, t6904: F, t1889: F, t5885: F, t5709: F, t1943: F, t28342: F, t27370: F, t27369: F, t28336: F, t28369: F, t28392: F, t28395: F, t29259: F, t29267: F, t7908: F, t8144: F, t8148: F, t8155: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t29270 = t7923 * t29269;
    let t29271 = t1394 * t29270;
    let t29273 = t5662 * t6281;
    let t29274 = t7923 * t29273;
    let t29275 = t4153 * t29274;
    let t29277 = t27387 * t7100;
    let t29278 = t1394 * t29277;
    let t29280 = t7923 * t6904;
    let t29281 = t1394 * t29280;
    let t29283 = t5885 * t1889;
    let t29284 = t5709 * t29283;
    let t29288 = t28342 * t1943;
    let t29289 = t27370 * t29288;
    let t29296 = -0.15445601851851851852e-3 * t28336 + 0.46336805555555555556e-3 * t7908 * t29259 - 0.46336805555555555556e-3 * t28369 * t8155 + 0.12356481481481481482e-2 * t28392 * t8155 + 0.33163888888888888888e-2 * t29267 + 0.16581944444444444444e-2 * t29271 + 0.27636574074074074073e-2 * t29275 - 0.33163888888888888888e-2 * t29278 + 0.22109259259259259258e-2 * t29281 + 0.61836467013888888889e-4 * t27369 * t29284 + 0.22109259259259259258e-2 * t28395 - 0.13901041666666666667e-2 * t7908 * t29289 - 0.18550940104166666667e-3 * t27369 * t29289 + 0.13901041666666666667e-2 * t8144 * t8148;
    (t29270, t29271, t29273, t29274, t29275, t29277, t29278, t29280, t29281, t29283, t29284, t29288, t29289, t29296)
}
