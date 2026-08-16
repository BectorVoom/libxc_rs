//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 456/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk456(t2030: f64, t2126: f64, t751: f64, t785: f64, t2036: f64, t306: f64, t287: f64, t314: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2127 = t2126 * t2030;
    let t2131 = t751 * t785;
    let t2138 = t2036 * t306;
    let t2139 = t2126 * t287;
    let t2155 = t314 * t314;
    let t2156 = 1.0_f64 / t2155;
    (t2127, t2131, t2138, t2139, t2155, t2156)
}
