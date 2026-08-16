//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1281/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1281(t7898: f64, t98524: f64, t27410: f64, t28426: f64, t3245: f64, t8176: f64, t27345: f64, t8144: f64, t1014: f64, t28409: f64, t27342: f64, t27396: f64, t27416: f64, t28397: f64, t8151: f64, t94664: f64, t94669: f64, t98600: f64) -> (f64, f64, f64) {
    let t98934 = t7898 * t98524;
    let t98938 = t27410 * t28426;
    let t98942 = t3245 * t8176;
    let t98945 = 0.46336805555555555556e-3_f64 * t8144 * t27345;
    let t98946 = t1014 * t28409;
    let t98950 = 0.16581944444444444444e-2_f64 * t94664 - 0.11054629629629629629e-2_f64 * t94669 + 0.92754700520833333333e-4_f64 * t7898 * t98600 - 0.20612155671296296296e-4_f64 * t98934 + 0.92754700520833333333e-4_f64 * t28397 * t27416 + 0.61836467013888888888e-4_f64 * t98938 - 0.13901041666666666667e-2_f64 * t8144 * t27342 + 0.14739506172839506172e-2_f64 * t98942 + t98945 - 0.5895802469135802469e-2_f64 * t98946 + 0.37069444444444444444e-2_f64 * t8151 * t27396;
    (t98942, t98946, t98950)
}
