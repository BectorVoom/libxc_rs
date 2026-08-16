//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3932/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3932(t10416: f64, t13426: f64, t13435: f64, t13521: f64, t13532: f64, t13540: f64, t13544: f64, t14310: f64, t18163: f64, t18232: f64, t18242: f64, t18245: f64, t1847: f64, t21658: f64, t21882: f64, t21891: f64, t2322: f64, t2372: f64, t4248: f64, t4254: f64, t4297: f64, t569: f64, t5887: f64, t5921: f64, t651: f64, t670: f64, t75672: f64, t7732: f64) -> f64 {
    let t75676 = -4.0_f64 * t21658 * t651 * t670 - 4.0_f64 * t10416 * t5887 - 8.0_f64 * t13426 * t4297 - 8.0_f64 * t13435 * t5887 - 4.0_f64 * t13521 * t4248 - 8.0_f64 * t13532 * t4248 - 8.0_f64 * t13532 * t7732 - 8.0_f64 * t13540 * t4248 - 4.0_f64 * t13544 * t4248 + 2.0_f64 * t14310 * t1847 - 2.0_f64 * t18163 * t5921 - 4.0_f64 * t18232 * t2322 - 4.0_f64 * t18242 * t4254 - 2.0_f64 * t18245 * t2372 - 4.0_f64 * t21882 * t2322 - 8.0_f64 * t21891 * t2322 + t569 * t75672;
    t75676
}
