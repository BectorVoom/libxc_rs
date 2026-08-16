//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1291/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1291(t20576: f64, t3726: f64, t16081: f64, t20586: f64, t20602: f64, t225: f64, t20420: f64, t20672: f64, t20670: f64, t1834: f64, t6414: f64, t20553: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t74747 = t3726 * t20576;
    let t74756 = t16081 * t20586;
    let t74849 = t20602 * t225;
    let t74860 = t20420 * t225;
    let t74908 = t20672 * t225;
    let t74930 = t20670 * t225;
    let t74937 = t1834 * t6414;
    let t74949 = t562 * t20553;
    (t74747, t74756, t74849, t74860, t74908, t74930, t74937, t74949)
}
