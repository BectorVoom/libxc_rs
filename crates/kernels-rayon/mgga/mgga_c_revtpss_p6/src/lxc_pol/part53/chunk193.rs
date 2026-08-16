//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 193/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk193(t235: f64, t240: f64, t234: f64, t243: f64, t807: f64, t236: f64, t786: f64, t27: f64, t124: f64, t800: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t808 = t235 * t240;
    let t810 = t234 * t808 * t243;
    let t812 = 0.71456696863449561619e-5_f64 * t807 * t810;
    let t813 = t786 * t236;
    let t814 = t27 * t240;
    let t815 = t814 * t243;
    let t816 = t800 * t124;
    (t808, t810, t812, t813, t814, t815, t816)
}
