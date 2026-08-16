//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1495/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1495(t10416: f64, t13426: f64, t13435: f64, t18153: f64, t18163: f64, t18227: f64, t1843: f64, t2198: f64, t2199: f64, t2322: f64, t27123: f64, t27126: f64, t31157: f64, t31172: f64, t31390: f64, t31407: f64, t3813: f64, t4254: f64, t651: f64, t7732: f64, t8307: f64, t8321: f64, t8327: f64, t8393: f64, t8406: f64, t8407: f64, t8411: f64, t98535: f64) -> f64 {
    let t117889 = -2.0_f64 * t18153 * t2198 * t651 - 2.0_f64 * t1843 * t31157 * t651 - 2.0_f64 * t3813 * t651 * t8406 - 2.0_f64 * t10416 * t8393 - 2.0_f64 * t10416 * t8407 + 2.0_f64 * t10416 * t8411 - 4.0_f64 * t13426 * t8321 + 4.0_f64 * t13426 * t8327 - 4.0_f64 * t13435 * t8393 - 4.0_f64 * t13435 * t8407 + 4.0_f64 * t13435 * t8411 - 2.0_f64 * t18163 * t8407 - 4.0_f64 * t18227 * t8321 - 2.0_f64 * t2199 * t98535 - 4.0_f64 * t2322 * t31390 - 4.0_f64 * t2322 * t31407 + 4.0_f64 * t27123 * t8327 - 4.0_f64 * t27126 * t8307 - 2.0_f64 * t31172 * t7732 - 4.0_f64 * t31407 * t4254;
    t117889
}
