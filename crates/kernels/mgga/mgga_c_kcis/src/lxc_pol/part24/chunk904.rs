//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 904/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk904<F: Float>(t19422: F, t19471: F, t19521: F, t19534: F, t10187: F, t10188: F, t10190: F, t1083: F, t13658: F, t13665: F, t13667: F, t13682: F, t13684: F, t13686: F, t13689: F, t14053: F, t14055: F, t1697: F, t1745: F, t18619: F, t19107: F, t19381: F, t278: F, t339: F, t4768: F, t4920: F, t6432: F, t6478: F, t975: F) -> (F, F) {
    let t19536 = t19422 + t19471 + t19521 + t19534;
    let t19539 = -F::new(0.93706135855523581992e-2) * t19381 - F::new(0.46853067927761790996e-2) * t10188 - F::new(0.93706135855523581992e-2) * t10190 - t13665 - t13667 - t13682 - t13684 + F::new(0.93706135855523581992e-2) * t13686 + t13689 - F::new(2.0) * t1697 * t4920 - F::new(2.0) * t4768 * t1745 - t975 * t6478 - F::new(0.28111840756657074598e-1) * t13658 * t18619 - t10187 - F::new(0.18741227171104716398e-1) * t14053 - F::new(0.93706135855523581992e-2) * t14055 - t6432 * t1083 - t278 * t19536 - t19107 * t339;
    (t19536, t19539)
}
