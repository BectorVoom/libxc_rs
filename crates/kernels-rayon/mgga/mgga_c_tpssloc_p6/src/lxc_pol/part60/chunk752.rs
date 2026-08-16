//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 752/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk752(t131: f64, t27505: f64, t467: f64, t225: f64, t8034: f64, t7327: f64, t24826: f64, t8074: f64, t7359: f64, t7999: f64, t1222: f64, t8043: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27506 = t27505 * t131;
    let t27507 = t27506 * t467;
    let t27516 = t8034 * t225;
    let t27536 = t8034 * t7327;
    let t27556 = t24826 * t8074;
    let t27572 = t7999 * t7359;
    let t27578 = t8043 * t1222;
    (t27507, t27516, t27536, t27556, t27572, t27578)
}
