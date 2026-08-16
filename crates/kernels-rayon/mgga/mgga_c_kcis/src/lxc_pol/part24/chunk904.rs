//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 904/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk904(t19422: f64, t19471: f64, t19521: f64, t19534: f64, t10187: f64, t10188: f64, t10190: f64, t1083: f64, t13658: f64, t13665: f64, t13667: f64, t13682: f64, t13684: f64, t13686: f64, t13689: f64, t14053: f64, t14055: f64, t1697: f64, t1745: f64, t18619: f64, t19107: f64, t19381: f64, t278: f64, t339: f64, t4768: f64, t4920: f64, t6432: f64, t6478: f64, t975: f64) -> (f64, f64) {
    let t19536 = t19422 + t19471 + t19521 + t19534;
    let t19539 = -0.93706135855523581992e-2_f64 * t19381 - 0.46853067927761790996e-2_f64 * t10188 - 0.93706135855523581992e-2_f64 * t10190 - t13665 - t13667 - t13682 - t13684 + 0.93706135855523581992e-2_f64 * t13686 + t13689 - 2.0_f64 * t1697 * t4920 - 2.0_f64 * t4768 * t1745 - t975 * t6478 - 0.28111840756657074598e-1_f64 * t13658 * t18619 - t10187 - 0.18741227171104716398e-1_f64 * t14053 - 0.93706135855523581992e-2_f64 * t14055 - t6432 * t1083 - t278 * t19536 - t19107 * t339;
    (t19536, t19539)
}
