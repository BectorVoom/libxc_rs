//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 481/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk481(t259: f64, t379: f64, t1812: f64, t30: f64, t1811: f64, t207: f64, t198: f64, t823: f64) -> (f64, f64, f64, f64) {
    let t380 = t259 < t379;
    let t1813 = t1812 * t30;
    let t1816 = t207 * t1811;
    let t1818 = t198 * t1816 * t823;
    let t1819 = piecewise3(t380, 0.0_f64, t1818);
    (t1813, t1816, t1818, t1819)
}
