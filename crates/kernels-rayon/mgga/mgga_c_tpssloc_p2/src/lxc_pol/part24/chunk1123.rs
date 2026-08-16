//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1123/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1123(t22776: f64, t6936: f64, t6604: f64, t6919: f64, t6937: f64, t6950: f64, t835: f64, t1336: f64, t1369: f64, t3876: f64, t6952: f64, t3777: f64, t6951: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22777 = t6936 * t22776;
    let t22779 = t6919 * t6604;
    let t22780 = t22779 * t6937;
    let t22782 = t6950 * t835;
    let t22783 = t1336 * t22782;
    let t22784 = t22783 * t1369;
    let t22785 = 7.0_f64 / 288.0_f64 * t22784;
    let t22786 = t6952 * t3876;
    let t22788 = t3777 * t6951;
    (t22777, t22779, t22780, t22782, t22783, t22785, t22786, t22788)
}
