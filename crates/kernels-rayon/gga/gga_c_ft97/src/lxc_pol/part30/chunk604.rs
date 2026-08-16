//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 604/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk604(t1154: f64, t713: f64, t2574: f64, t6119: f64, t24437: f64, t1091: f64, t2354: f64, t24546: f64, t6118: f64, t1433: f64, t3051: f64, t3746: f64) -> (f64, f64, f64, f64, f64) {
    let t27796 = t1154 * t713;
    let t27798 = t2574 * t6119 * t27796;
    let t27799 = t24437 * t27798;
    let t27802 = t2354 * t24546 * t1091;
    let t27803 = t6118 * t27802;
    let t27805 = t1433 * t3051;
    let t27807 = t2354 * t6119 * t3746;
    (t27796, t27799, t27803, t27805, t27807)
}
