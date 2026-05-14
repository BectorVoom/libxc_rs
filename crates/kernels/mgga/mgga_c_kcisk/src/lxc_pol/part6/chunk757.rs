//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 757/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk757<F: Float>(t10519: F, t10520: F, t28312: F, t8: F, t1899: F, t1800: F, t1869: F, t22250: F, t2528: F, t23286: F, t28277: F, t28282: F, t28285: F, t28288: F, t28292: F, t28297: F, t28301: F, t28306: F, t28309: F) -> (F, F, F, F) {
    let t28314 = t28312 * t8 - t10519 - t10520;
    let t28315 = t1899 * t28314;
    let t28316 = t1800 * t28315;
    let t28317 = t1869 * t28316;
    let t28319 = t22250 * t2528;
    let t28320 = t1869 * t28319;
    let t28323 = -0.49745833333333333332e-2 * t28277 + 0.33163888888888888887e-2 * t28282 - 0.99491666666666666664e-2 * t28285 - 0.2653111111111111111e-1 * t28288 + 0.2653111111111111111e-1 * t28292 - 0.49745833333333333332e-2 * t28297 + 0.48640370370370370369e-1 * t28301 + 0.16581944444444444444e-2 * t28306 + 0.49745833333333333332e-2 * t28309 + 0.16581944444444444444e-2 * t28317 - 0.74618749999999999998e-2 * t28320 - 0.66327777777777777776e-2 * t23286;
    (t28314, t28317, t28320, t28323)
}
