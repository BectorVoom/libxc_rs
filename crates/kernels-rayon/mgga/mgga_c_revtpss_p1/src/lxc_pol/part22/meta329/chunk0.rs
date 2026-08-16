//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1783/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1783(t10905: f64, t2732: f64, t136: f64, t860: f64, t2457: f64, t2710: f64, t10652: f64, t231: f64, t2783: f64, t2782: f64, t10069: f64, t2786: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10906 = t10905 * t2732;
    let t10914 = t860 * t136;
    let t10916 = t2710 * t10914 * t2457;
    let t10920 = t2783 * t10652 * t231;
    let t10921 = t2782 * t10920;
    let t10923 = t10069 * t2786;
    (t10906, t10914, t10916, t10920, t10921, t10923)
}
