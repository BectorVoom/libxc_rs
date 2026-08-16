//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1644/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1644(t1390: f64, t6874: f64, t828: f64, t4012: f64, t6836: f64, t124: f64, t6816: f64) -> (f64, f64, f64) {
    let t6876 = t1390 * t828 * t6874;
    let t6880 = t4012 * t828 * t6836;
    let t6883 = t124 * t6816;
    (t6876, t6880, t6883)
}
