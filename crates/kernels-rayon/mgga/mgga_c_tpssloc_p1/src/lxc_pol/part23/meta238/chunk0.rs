//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 892/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk892(t5723: f64, t699: f64, t5769: f64, t942: f64, t5737: f64, t923: f64, t2932: f64, t5790: f64, t10632: f64, t5774: f64, t2844: f64, t5726: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17290 = t699 * t5723;
    let t17355 = t5769 * t942;
    let t17428 = t5737 * t923;
    let t17492 = t5790 * t2932;
    let t17499 = t5774 * t10632;
    let t17520 = t5726 * t2844;
    (t17290, t17355, t17428, t17492, t17499, t17520)
}
