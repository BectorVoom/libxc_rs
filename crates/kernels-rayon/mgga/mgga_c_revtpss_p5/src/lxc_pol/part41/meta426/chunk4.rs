//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1490/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1490(t114: f64, t118353: f64, t118405: f64, t13426: f64, t18227: f64, t1843: f64, t21658: f64, t2178: f64, t2181: f64, t2322: f64, t30138: f64, t31248: f64, t31292: f64, t31293: f64, t31299: f64, t31320: f64, t31324: f64, t31518: f64, t31570: f64, t31579: f64, t4248: f64, t4254: f64, t508: f64, t5517: f64, t651: f64, t75439: f64, t7732: f64, t7889: f64, t8274: f64, t8353: f64, t8362: f64, t8367: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t118407 = piecewise3(t115, 0.0_f64, t118353 + t118405);
    let t118413 = -2.0_f64 * t118407 * t508 * t651 - 4.0_f64 * t1843 * t31292 * t651 - 2.0_f64 * t21658 * t2178 * t651 - 4.0_f64 * t5517 * t651 * t8362 - 4.0_f64 * t13426 * t8353 + 4.0_f64 * t13426 * t8367 - 4.0_f64 * t18227 * t8353 + 4.0_f64 * t18227 * t8367 + 2.0_f64 * t2181 * t75439 - 2.0_f64 * t2322 * t31518 + 4.0_f64 * t2322 * t31570 - 4.0_f64 * t2322 * t31579 - 4.0_f64 * t30138 * t8274 + 4.0_f64 * t31248 * t7889 + 4.0_f64 * t31293 * t4248 - 4.0_f64 * t31299 * t4248 - 4.0_f64 * t31320 * t7732 + 4.0_f64 * t31324 * t7889 - 2.0_f64 * t31518 * t4254 - 4.0_f64 * t31579 * t4254;
    (t118407, t118413)
}
