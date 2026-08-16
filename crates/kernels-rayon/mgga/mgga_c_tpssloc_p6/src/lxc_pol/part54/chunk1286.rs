//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1286/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1286(t2047: f64, t213: f64, t225: f64, t31351: f64, t794: f64, t6562: f64, t6572: f64, t82133: f64, t8547: f64, t7106: f64, t857: f64, t31362: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t114770 = t213 * t2047 * t225;
    let t114785 = t31351 * t225;
    let t114790 = t794 * t2047;
    let t114792 = t6562 * t114790 * t6572;
    let t114795 = t6562 * t82133 * t8547;
    let t114797 = t857 * t7106;
    let t114811 = t31362 * t225;
    (t114770, t114785, t114790, t114792, t114795, t114797, t114811)
}
