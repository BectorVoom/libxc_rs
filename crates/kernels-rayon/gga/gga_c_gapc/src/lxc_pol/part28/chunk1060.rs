//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1060/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1060(t11792: f64, t11796: f64, t11800: f64, t11806: f64, t11809: f64, t11811: f64, t11816: f64, t11818: f64, t11820: f64, t11823: f64, t11829: f64, t11832: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12213 = 0.34752370105806885418e-3_f64 * t11792;
    let t12214 = 0.51491428373437201895e-5_f64 * t11796;
    let t12215 = 0.21720231316129303386e-4_f64 * t11800;
    let t12216 = 0.24581606547037760418e-8_f64 * t11806;
    let t12217 = 0.35170937063461460537e-8_f64 * t11809;
    let t12218 = 0.33147827249531850013e-7_f64 * t11811;
    let t12219 = 0.12290803273518880209e-8_f64 * t11816;
    let t12220 = 0.32042899674547455013e-6_f64 * t11818;
    let t12221 = 0.11254699860307667372e-6_f64 * t11820;
    let t12222 = 0.30353495895471971565e-6_f64 * t11823;
    let t12224 = 0.12290803273518880209e-8_f64 * t11829;
    let t12225 = 0.8193868849012586806e-9_f64 * t11832;
    (t12213, t12214, t12215, t12216, t12217, t12218, t12219, t12220, t12221, t12222, t12224, t12225)
}
