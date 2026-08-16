//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 989/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk989(t2341: f64, t92: f64, t2219: f64, t659: f64, t2248: f64, t4049: f64, t584: f64, t95: f64, t16: f64, t4053: f64, t1449: f64, t2350: f64, t9398: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12774 = t92 * t2341;
    let t12775 = t2219 * t659;
    let t12778 = t4049 * t2248;
    let t12781 = t95 * t584;
    let t12784 = t4053 * t16;
    let t12792 = t9398 * t1449 * t2350;
    (t12774, t12775, t12778, t12781, t12784, t12792)
}
