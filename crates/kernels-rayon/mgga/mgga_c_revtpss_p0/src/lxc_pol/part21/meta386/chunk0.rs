//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1816/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1816(t1211: f64, t12621: f64, t1207: f64, t456: f64) -> (f64, f64, f64, f64) {
    let t12622 = t1211 * t12621;
    let t12625 = t1207 * t1207;
    let t12626 = 1.0_f64 / t12625;
    let t12627 = t456 * t12626;
    (t12622, t12625, t12626, t12627)
}
