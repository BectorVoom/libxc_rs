//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 823/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk823(t24115: f64, t24137: f64, t1378: f64, t1323: f64, t7191: f64, t1385: f64, t7213: f64, t3887: f64, t22923: f64, t22925: f64, t2085: f64, t3752: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24138 = t24115 + t24137;
    let t24139 = t1378 * t24138;
    let t24141 = t1323 * t7191;
    let t24146 = t7213 * t1385;
    let t24147 = t3887 * t24146;
    let t24156 = 0.12793931631041761173e0_f64 * t22923;
    let t24157 = 0.52089578783527170489e-1_f64 * t22925;
    let t24162 = t3752 * t2085;
    (t24138, t24139, t24141, t24147, t24156, t24157, t24162)
}
