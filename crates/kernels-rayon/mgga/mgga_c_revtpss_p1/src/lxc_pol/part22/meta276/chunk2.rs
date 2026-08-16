//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1680/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1680(t1340: f64, t9419: f64, t2626: f64, t4038: f64, t2491: f64, t745: f64, t9368: f64) -> (f64, f64, f64) {
    let t9421 = 0.10389515463408878255e3_f64 * t1340 * t9419;
    let t9422 = t4038 * t2626;
    let t9425 = t2491 * t9368 * t745;
    (t9421, t9422, t9425)
}
