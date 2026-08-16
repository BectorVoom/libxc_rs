//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 935/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk935(t3994: f64, t9794: f64, t9793: f64, t2713: f64, t3951: f64, t3964: f64, t785: f64, t9731: f64) -> (f64, f64, f64) {
    let t9795 = t9794 * t3994;
    let t9796 = t9793 * t9795;
    let t9799 = t3964 * t2713 * t3951;
    let t9801 = t9731 * t785;
    (t9796, t9799, t9801)
}
