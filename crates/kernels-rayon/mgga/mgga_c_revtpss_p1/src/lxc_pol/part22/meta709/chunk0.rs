//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2732/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2732(t26: f64, t65: f64, t1868: f64, t4147: f64, t1651: f64, t385: f64, t1774: f64, t494: f64, t9163: f64, t99: f64, t107: f64, t9232: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33127 = 1.0_f64 / t65 / t26;
    let t33596 = t4147 * t1868;
    let t33754 = t385 * t1651;
    let t34934 = t494 * t1774;
    let t36227 = t99 * t9163;
    let t36415 = t107 * t9232;
    (t33127, t33596, t33754, t34934, t36227, t36415)
}
