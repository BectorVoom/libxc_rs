//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1262/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1262(t17303: f64, t7613: f64, t26866: f64, t5436: f64, t17361: f64, t7618: f64, t17307: f64, t2138: f64, t3682: f64, t8172: f64, t3655: f64, t8185: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104825 = t7613 * t17303;
    let t104888 = t5436 * t26866;
    let t104905 = t7618 * t17361;
    let t104927 = t17307 * t2138;
    let t104963 = t8172 * t3682;
    let t104988 = t8185 * t3655;
    (t104825, t104888, t104905, t104927, t104963, t104988)
}
