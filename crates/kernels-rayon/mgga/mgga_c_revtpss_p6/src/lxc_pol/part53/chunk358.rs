//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 358/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk358(t1626: f64, t324: f64, t1594: f64, t1601: f64, t1604: f64, t1607: f64, t967: f64, t970: f64) -> (f64, f64) {
    let t1627 = t1626 * t324;
    let t1633 = 0.258925e1_f64 * t1601 - t967 - 0.301925e0_f64 * t1594 + 0.16504875e0_f64 * t1604 - t970 - 0.82785e-1_f64 * t1607;
    (t1627, t1633)
}
