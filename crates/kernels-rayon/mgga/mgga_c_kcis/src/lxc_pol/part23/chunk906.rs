//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 906/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk906(t16919: f64, t3984: f64, t12147: f64, t5722: f64, t1368: f64, t531: f64, t5732: f64, t833: f64, t5705: f64, t12135: f64, t12138: f64, t12142: f64, t12152: f64, t16902: f64, t16907: f64, t16911: f64, t3986: f64, t5691: f64) -> f64 {
    let t16920 = t3984 * t16919;
    let t16923 = t12147 * t5722;
    let t16925 = t1368 * t16923 / 432.0_f64;
    let t16926 = t5732 * t531;
    let t16927 = t16926 * t833;
    let t16928 = t3984 * t16927;
    let t16933 = t12147 * t5705;
    let t16935 = t1368 * t16933 / 432.0_f64;
    let t16936 = 7.0_f64 / 648.0_f64 * t1368 * t16902 - t1368 * t16907 / 54.0_f64 - t1368 * t16911 / 288.0_f64 - t12135 / 648.0_f64 + t12138 / 864.0_f64 + t12142 / 648.0_f64 - t12152 / 432.0_f64 + t1368 * t16920 / 144.0_f64 - t16925 - t1368 * t16928 / 144.0_f64 + t5691 * t3986 / 54.0_f64 - t16935;
    t16936
}
