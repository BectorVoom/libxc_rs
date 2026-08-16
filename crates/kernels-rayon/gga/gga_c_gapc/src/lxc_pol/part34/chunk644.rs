//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 644/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk644(t3751: f64, t3752: f64, t122: f64, t825: f64, t125: f64, t311: f64) -> (f64, f64, f64, f64) {
    let t3753 = t3751 * t3752;
    let t3755 = t825 * t122;
    let t3756 = t3755 * t125;
    let t3757 = t311 * t3756;
    (t3753, t3755, t3756, t3757)
}
