//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1477/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1477(t31555: f64, t508: f64, t569: f64, t1911: f64, t8362: f64, t1843: f64, t1312: f64, t18245: f64, t2179: f64, t2181: f64, t29508: f64, t30138: f64, t30143: f64, t31518: f64, t31533: f64, t4248: f64, t651: f64, t7732: f64, t7889: f64, t8353: f64, t8363: f64, t8367: f64, t8369: f64) -> (f64, f64, f64, f64, f64) {
    let t31556 = t508 * t31555;
    let t31567 = t31555 * t569;
    let t31570 = t8362 * t1911;
    let t31579 = t1843 * t8362;
    let t31582 = 2.0_f64 * t1312 * t31533 + 2.0_f64 * t1312 * t31567 + 4.0_f64 * t1312 * t31570 - 2.0_f64 * t18245 * t2179 + 2.0_f64 * t18245 * t2181 - 2.0_f64 * t2179 * t29508 - 4.0_f64 * t2179 * t30138 + 4.0_f64 * t2181 * t30138 + 2.0_f64 * t2181 * t30143 - 2.0_f64 * t31518 * t651 - 2.0_f64 * t31556 * t651 - 4.0_f64 * t31579 * t651 - 4.0_f64 * t4248 * t8353 - 4.0_f64 * t4248 * t8363 + 4.0_f64 * t4248 * t8367 + 4.0_f64 * t4248 * t8369 - 4.0_f64 * t7732 * t8353 - 4.0_f64 * t7732 * t8363 + 4.0_f64 * t7889 * t8367 + 4.0_f64 * t7889 * t8369;
    (t31556, t31567, t31570, t31579, t31582)
}
