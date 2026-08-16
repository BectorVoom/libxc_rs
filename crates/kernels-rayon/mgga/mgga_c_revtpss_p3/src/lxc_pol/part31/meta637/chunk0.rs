//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2092/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2092(t15654: f64, t1976: f64, t27708: f64, t3336: f64, t11108: f64, t7840: f64, t33: f64, t41154: f64, t1711: f64, t2411: f64, t28150: f64, t6973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100760 = t15654 * t1976;
    let t100802 = t27708 * t3336;
    let t100806 = t7840 * t11108;
    let t100981 = t41154 * t33;
    let t100987 = t2411 * t1711;
    let t101211 = t6973 * t28150;
    (t100760, t100802, t100806, t100981, t100987, t101211)
}
