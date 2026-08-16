//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1499/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1499(t10416: f64, t1312: f64, t13426: f64, t13435: f64, t1453: f64, t18227: f64, t1843: f64, t2179: f64, t2181: f64, t2322: f64, t27123: f64, t27126: f64, t31066: f64, t31070: f64, t31248: f64, t31292: f64, t31314: f64, t31318: f64, t4248: f64, t4254: f64, t49686: f64, t5523: f64, t651: f64, t75485: f64, t75667: f64, t8254: f64, t8278: f64, t8280: f64, t8363: f64, t98535: f64) -> f64 {
    let t117622 = 4.0_f64 * t1312 * t1453 * t31292 - 2.0_f64 * t1843 * t31066 * t651 - 2.0_f64 * t10416 * t8363 - 4.0_f64 * t13426 * t8254 + 4.0_f64 * t13426 * t8278 - 4.0_f64 * t13435 * t8363 + 4.0_f64 * t18227 * t8278 + 4.0_f64 * t18227 * t8280 - 2.0_f64 * t2179 * t49686 - 4.0_f64 * t2179 * t75667 - 2.0_f64 * t2179 * t98535 + 2.0_f64 * t2181 * t49686 + 2.0_f64 * t2181 * t75485 + 4.0_f64 * t2181 * t75667 - 4.0_f64 * t2322 * t31318 + 4.0_f64 * t27123 * t8280 - 4.0_f64 * t27126 * t8254 + 4.0_f64 * t31070 * t4248 + 4.0_f64 * t31248 * t5523 - 4.0_f64 * t31314 * t4254;
    t117622
}
