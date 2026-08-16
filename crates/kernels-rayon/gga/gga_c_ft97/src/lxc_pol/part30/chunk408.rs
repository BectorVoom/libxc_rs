//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 408/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk408(t11: f64, t6817: f64, t14: f64, t213: f64, t231: f64, t1127: f64, t6045: f64, t1103: f64, t444: f64, t1419: f64, t1091: f64, t2917: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6818 = t11 * t6817;
    let t6819 = t6818 * t14;
    let t6820 = t231 * t213;
    let t6821 = t6819 * t6820;
    let t6824 = t231 * t1127;
    let t6825 = t6045 * t6824;
    let t6828 = t444 * t1103;
    let t6829 = t6828 * t1419;
    let t6832 = t2917 * t1091;
    (t6818, t6819, t6820, t6821, t6824, t6825, t6828, t6829, t6832)
}
