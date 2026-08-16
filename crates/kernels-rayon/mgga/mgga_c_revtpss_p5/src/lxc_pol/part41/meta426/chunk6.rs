//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1492/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1492(t108710: f64, t108714: f64, t109150: f64, t109153: f64, t118407: f64, t1312: f64, t13426: f64, t18227: f64, t1911: f64, t2179: f64, t2181: f64, t2322: f64, t29508: f64, t30138: f64, t30143: f64, t31292: f64, t31309: f64, t31320: f64, t31533: f64, t31567: f64, t31570: f64, t4248: f64, t5523: f64, t569: f64, t6934: f64, t8254: f64, t8273: f64, t8274: f64, t8278: f64, t8280: f64, t8369: f64) -> f64 {
    let t118500 = 2.0_f64 * t118407 * t1312 * t569 + 4.0_f64 * t1312 * t1911 * t31292 + 2.0_f64 * t1312 * t6934 * t8273 - 2.0_f64 * t108710 * t2179 - 2.0_f64 * t108714 * t2179 + 4.0_f64 * t109150 * t2181 + 4.0_f64 * t109153 * t2181 + 4.0_f64 * t13426 * t8369 + 4.0_f64 * t18227 * t8369 + 2.0_f64 * t2322 * t31533 + 2.0_f64 * t2322 * t31567 - 2.0_f64 * t29508 * t8254 - 2.0_f64 * t29508 * t8274 + 4.0_f64 * t30138 * t8278 + 2.0_f64 * t30143 * t8280 + 4.0_f64 * t31309 * t4248 - 4.0_f64 * t31320 * t4248 + 2.0_f64 * t31533 * t5523 + 2.0_f64 * t31567 * t5523 + 4.0_f64 * t31570 * t5523;
    t118500
}
