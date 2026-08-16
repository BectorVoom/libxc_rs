//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 537/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk537(t218: f64, t219: f64, t2226: f64, t2185: f64, t334: f64, t2175: f64, t2187: f64, t2205: f64, t2210: f64, t2212: f64, t2216: f64, t2218: f64, t2222: f64, t2224: f64) -> (f64, f64, f64, f64) {
    let t2228 = t218 * t219 * t2226;
    let t2230 = t334 * t2185;
    let t2232 = t218 * t219 * t2230;
    let t2234 = -0.9494625e0_f64 * t2205 + 0.1898925e1_f64 * t2210 + t2212 - 0.59793333333333333334e0_f64 * t2175 + 0.8969e0_f64 * t2187 + 0.15358125e0_f64 * t2216 + 0.3071625e0_f64 * t2218 + t2222 - 0.32862666666666666666e0_f64 * t2224 + 0.24647e0_f64 * t2228 + 0.24647e0_f64 * t2232;
    (t2228, t2230, t2232, t2234)
}
