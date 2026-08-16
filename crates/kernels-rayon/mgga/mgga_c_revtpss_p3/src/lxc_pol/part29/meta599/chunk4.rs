//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2043/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2043(t10416: f64, t13426: f64, t13435: f64, t18153: f64, t18227: f64, t1843: f64, t2014: f64, t2052: f64, t2320: f64, t2322: f64, t25089: f64, t25188: f64, t26153: f64, t26376: f64, t26380: f64, t26396: f64, t26406: f64, t27833: f64, t28196: f64, t28286: f64, t28586: f64, t28704: f64, t28709: f64, t28938: f64, t4248: f64, t5542: f64, t649: f64, t651: f64, t7235: f64, t7374: f64, t7489: f64, t7539: f64, t7898: f64, t7984: f64, t8065: f64, t8109: f64, t98450: f64, t98550: f64) -> f64 {
    let t104038 = t25188 * t8109 - 4.0_f64 * t13426 * t7374 - 4.0_f64 * t18227 * t7374 - 4.0_f64 * t4248 * t26396 - 2.0_f64 * t7235 * t28709 + 3.0_f64 * t2014 * t28938 * t25089 - 2.0_f64 * t7898 * t26380 - 6.0_f64 * t98450 * t26406 - t2052 * t18153 + 2.0_f64 * t28196 * t28286 * t98550 + 6.0_f64 * t27833 * t7489 - t2014 * t26376 * t5542 - t2320 * t8065 - 2.0_f64 * t649 * t28586 - 2.0_f64 * t27833 * t7539 - 2.0_f64 * t651 * t1843 * t26153 - 2.0_f64 * t10416 * t7984 - 4.0_f64 * t13435 * t7984 - 4.0_f64 * t2322 * t28704;
    t104038
}
