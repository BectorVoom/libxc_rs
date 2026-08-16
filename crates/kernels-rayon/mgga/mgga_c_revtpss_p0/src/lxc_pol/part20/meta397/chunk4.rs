//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1469/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1469(t11408: f64, t941: f64, t2979: f64, t2986: f64, t11465: f64, t960: f64, t11585: f64, t945: f64, t2935: f64, t2967: f64, t11509: f64, t3006: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41779 = t941 * t11408;
    let t41785 = t2979 * t2986;
    let t41788 = t960 * t11465;
    let t41794 = t11585 * t945;
    let t41799 = t2935 * t2967;
    let t41813 = t3006 * t11509;
    (t41779, t41785, t41788, t41794, t41799, t41813)
}
