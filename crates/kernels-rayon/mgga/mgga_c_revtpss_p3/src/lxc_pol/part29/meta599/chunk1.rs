//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2040/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2040(t102714: f64, t10416: f64, t13435: f64, t1519: f64, t18153: f64, t18163: f64, t1911: f64, t2014: f64, t2055: f64, t2106: f64, t2322: f64, t2371: f64, t25082: f64, t26377: f64, t26383: f64, t26392: f64, t26399: f64, t26405: f64, t26699: f64, t27153: f64, t28167: f64, t28704: f64, t28750: f64, t28760: f64, t33183: f64, t3829: f64, t4254: f64, t4257: f64, t651: f64, t7898: f64, t7900: f64, t7978: f64, t7984: f64, t7988: f64, t8065: f64, t95357: f64, t98519: f64) -> f64 {
    let t103917 = -2.0_f64 * t18163 * t7984 - 4.0_f64 * t4254 * t28704 - 6.0_f64 * t28167 * t26405 * t98519 - 6.0_f64 * t25082 * t33183 * t27153 + t7898 * t26377 + 6.0_f64 * t2014 * t3829 * t2106 * t7900 + 3.0_f64 * t7898 * t26383 + t26699 * t1911 - 2.0_f64 * t651 * t18153 * t2055 - 2.0_f64 * t651 * t8065 * t2371 - 2.0_f64 * t95357 * t1519 - 4.0_f64 * t102714 * t1519 - 4.0_f64 * t26399 * t4257 - 2.0_f64 * t18163 * t7978 - 4.0_f64 * t4254 * t28760 - 2.0_f64 * t10416 * t7988 - 4.0_f64 * t13435 * t7988 - 4.0_f64 * t2322 * t28750 - t7898 * t26392;
    t103917
}
