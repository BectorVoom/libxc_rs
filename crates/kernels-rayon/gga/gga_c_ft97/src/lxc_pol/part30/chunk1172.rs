//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1172/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1172(t1882: f64, t36175: f64, t36002: f64, t870: f64, t875: f64, t36246: f64, t36211: f64, t36157: f64, t8392: f64, t36257: f64, t25253: f64, t7124: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t154787 = t1882 * t36175;
    let t154793 = t36002 * t870;
    let t154794 = t154793 * t875;
    let t154807 = t1882 * t36246;
    let t154813 = t1882 * t36211;
    let t154820 = t8392 * t36157;
    let t154827 = t1882 * t36257;
    let t154833 = t25253 * t7124;
    (t154787, t154794, t154807, t154813, t154820, t154827, t154833)
}
