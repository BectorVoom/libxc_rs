//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1067/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1067(t10309: f64, t124455: f64, t2247: f64, t10301: f64, t33362: f64, t33358: f64, t45972: f64, t45963: f64, t116: f64, t33374: f64, t33474: f64, t33508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t124456 = t10309 * t124455;
    let t124463 = t2247 * t124455;
    let t124480 = t10301 * t33362;
    let t124483 = t45972 * t33358;
    let t124503 = t45963 * t33358;
    let t124508 = t10301 * t33358;
    let t124533 = t33374 * t116;
    let t124554 = t33474 * t33508;
    (t124456, t124463, t124480, t124483, t124503, t124508, t124533, t124554)
}
