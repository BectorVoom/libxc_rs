//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1410/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1410(t1340: f64, t9318: f64, t2491: f64, t2514: f64, t2495: f64, t744: f64) -> (f64, f64, f64) {
    let t9320 = 0.35089341735807877242e1_f64 * t1340 * t9318;
    let t9321 = t2491 * t2514;
    let t9323 = t9321 * t2495 * t744;
    (t9320, t9321, t9323)
}
