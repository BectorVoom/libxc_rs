//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2326/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2326(t1501: f64, t1518: f64, t10208: f64, t69: f64, t26: f64, t65: f64, t1651: f64, t385: f64, t1774: f64, t494: f64, t9163: f64, t99: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30138 = t1501 * t1518;
    let t31035 = t69 * t10208;
    let t33127 = 1.0_f64 / t65 / t26;
    let t33754 = t385 * t1651;
    let t34934 = t494 * t1774;
    let t36227 = t99 * t9163;
    (t30138, t31035, t33127, t33754, t34934, t36227)
}
