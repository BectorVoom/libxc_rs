//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1132/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1132(t4038: f64, t762: f64, t1340: f64, t2626: f64) -> (f64, f64, f64) {
    let t4039 = t4038 * t762;
    let t4040 = 0.11696447245269292414e1_f64 * t4039;
    let t4042 = 0.11696447245269292414e1_f64 * t1340 * t2626;
    (t4039, t4040, t4042)
}
