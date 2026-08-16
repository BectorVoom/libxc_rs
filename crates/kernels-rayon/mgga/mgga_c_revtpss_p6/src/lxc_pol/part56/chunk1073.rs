//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1073/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1073(t124645: f64, t372: f64, t124610: f64, t12808: f64, t3566: f64, t7657: f64, t1032: f64, t2142: f64, t2148: f64, t26916: f64, t7642: f64, t33468: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t124646 = t372 * t124645;
    let t124650 = t12808 * t124610;
    let t124659 = t3566 * t7657;
    let t124664 = t2142 * t1032;
    let t124665 = t2148 * t124664;
    let t124668 = t7642 * t26916;
    let t124671 = t33468 * t26916;
    (t124646, t124650, t124659, t124664, t124665, t124668, t124671)
}
