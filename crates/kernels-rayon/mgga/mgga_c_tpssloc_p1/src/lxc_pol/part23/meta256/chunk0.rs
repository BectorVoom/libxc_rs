//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 917/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk917(t5480: f64, t9398: f64, t6320: f64, t67: f64, t758: f64, t12061: f64, t6305: f64, t12072: f64, t6312: f64, t750: f64, t17: f64, t588: f64, t6328: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19513 = t9398 * t5480;
    let t19541 = t6320 * t67;
    let t19542 = t19541 * t758;
    let t19547 = t12061 * t6305;
    let t19559 = t12072 * t6312;
    let t19575 = t6320 * t750;
    let t19576 = t17 * t19575;
    let t19591 = t588 * t6328;
    (t19513, t19541, t19542, t19547, t19559, t19575, t19576, t19591)
}
