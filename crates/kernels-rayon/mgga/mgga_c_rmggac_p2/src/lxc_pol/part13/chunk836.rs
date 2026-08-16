//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 836/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk836(t2347: f64, t4905: f64, t26287: f64, t2350: f64, t798: f64, t31057: f64, t4048: f64, t7494: f64, t8526: f64, t2060: f64, t5249: f64, t739: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t38795 = t2347 * t4905;
    let t38796 = t26287 * t38795;
    let t38798 = t2350 * t798;
    let t38799 = t31057 * t38798;
    let t38801 = t2350 * t4048;
    let t38802 = t26287 * t38801;
    let t38807 = t7494 * t8526;
    let t38812 = t2060 * t5249;
    let t38813 = t739 * t38812;
    (t38795, t38796, t38798, t38799, t38801, t38802, t38807, t38812, t38813)
}
