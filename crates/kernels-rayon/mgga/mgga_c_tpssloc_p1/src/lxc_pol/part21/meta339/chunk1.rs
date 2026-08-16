//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1726/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1726(t584: f64, t95: f64, t16: f64, t4053: f64, t1449: f64, t2350: f64, t9398: f64, t100: f64, t2349: f64, t2219: f64, t662: f64, t2354: f64, t4059: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12781 = t95 * t584;
    let t12784 = t4053 * t16;
    let t12792 = t9398 * t1449 * t2350;
    let t12795 = t100 * t2349;
    let t12796 = t2219 * t662;
    let t12799 = t4059 * t2354;
    (t12781, t12784, t12792, t12795, t12796, t12799)
}
