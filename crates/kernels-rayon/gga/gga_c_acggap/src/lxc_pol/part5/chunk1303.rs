//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1303/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1303(t14173: f64, t6144: f64, t1181: f64, t3361: f64, t4623: f64, t530: f64, t3382: f64, t5796: f64, t1140: f64, t5586: f64, t1017: f64, t1165: f64, t13299: f64, t13364: f64, t17185: f64, t1849: f64, t18768: f64, t18770: f64, t18772: f64, t20305: f64, t22705: f64, t336: f64, t3396: f64, t360: f64, t367: f64, t372: f64, t4261: f64, t4762: f64, t6138: f64, t8790: f64, t8927: f64) -> f64 {
    let t24211 = t14173 * t6144;
    let t24218 = t3361 * t1181 * t530 * t4623;
    let t24220 = t3382 * t5796;
    let t24222 = t1140 * t5586;
    let t24242 = -0.20579528696673473746e-1_f64 * t3396 * t1165 * t6138 * t4762 + 0.34299214494455789578e-2_f64 * t24211 + 0.45351183609335988443e-1_f64 * t18768 + 0.90702367218671976884e-1_f64 * t18770 - 0.80031500487063509015e-2_f64 * t18772 + 0.68598428988911579156e-2_f64 * t24218 - 0.17149607247227894789e-2_f64 * t24220 + 7.0_f64 / 144.0_f64 * t24222 + t367 * t336 * t22705 * t1017 / 48.0_f64 + t4261 * t8927 * t1849 * t1017 / 4.0_f64 - 0.13719685797782315831e-1_f64 * t17185 * t13364 * t8790 * t20305 * t360 + 0.13719685797782315831e-1_f64 * t17185 * t13299 * t8790 * t20305 * t372;
    t24242
}
