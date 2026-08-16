//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 843/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk843(t10519: f64, t10520: f64, t28312: f64, t8: f64, t1899: f64, t1800: f64, t1869: f64, t22250: f64, t2528: f64, t23286: f64, t28277: f64, t28282: f64, t28285: f64, t28288: f64, t28292: f64, t28297: f64, t28301: f64, t28306: f64, t28309: f64) -> (f64, f64, f64, f64) {
    let t28314 = t28312 * t8 - t10519 - t10520;
    let t28315 = t1899 * t28314;
    let t28316 = t1800 * t28315;
    let t28317 = t1869 * t28316;
    let t28319 = t22250 * t2528;
    let t28320 = t1869 * t28319;
    let t28323 = -0.49745833333333333332e-2_f64 * t28277 + 0.33163888888888888887e-2_f64 * t28282 - 0.99491666666666666664e-2_f64 * t28285 - 0.2653111111111111111e-1_f64 * t28288 + 0.2653111111111111111e-1_f64 * t28292 - 0.49745833333333333332e-2_f64 * t28297 + 0.48640370370370370369e-1_f64 * t28301 + 0.16581944444444444444e-2_f64 * t28306 + 0.49745833333333333332e-2_f64 * t28309 + 0.16581944444444444444e-2_f64 * t28317 - 0.74618749999999999998e-2_f64 * t28320 - 0.66327777777777777776e-2_f64 * t23286;
    (t28314, t28317, t28320, t28323)
}
