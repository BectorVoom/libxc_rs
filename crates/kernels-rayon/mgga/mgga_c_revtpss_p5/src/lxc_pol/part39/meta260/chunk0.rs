//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 971/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk971(t508: f64, t8362: f64, t569: f64, t1911: f64, t2178: f64, t1312: f64, t2179: f64, t2181: f64, t4248: f64, t651: f64, t7732: f64, t7889: f64, t8353: f64) -> (f64, f64, f64, f64) {
    let t8363 = t508 * t8362;
    let t8367 = t8362 * t569;
    let t8369 = t2178 * t1911;
    let t8372 = 2.0_f64 * t1312 * t8367 + 2.0_f64 * t1312 * t8369 - 2.0_f64 * t2179 * t4248 - 2.0_f64 * t2179 * t7732 + 2.0_f64 * t2181 * t4248 + 2.0_f64 * t2181 * t7889 - 2.0_f64 * t651 * t8353 - 2.0_f64 * t651 * t8363;
    (t8363, t8367, t8369, t8372)
}
