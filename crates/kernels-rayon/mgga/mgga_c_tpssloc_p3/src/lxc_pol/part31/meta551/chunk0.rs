//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1779/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1779(t1883: f64, t82045: f64, t23012: f64, t6568: f64, t23205: f64, t82038: f64, t1081: f64, t2752: f64, t608: f64, t9239: f64, t22573: f64, t6875: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82218 = t82045 * t1883;
    let t82259 = t23012 * t6568;
    let t82294 = t82038 * t23205;
    let t83555 = t2752 * t1081;
    let t83717 = t9239 * t608;
    let t83886 = t6875 * t22573;
    (t82218, t82259, t82294, t83555, t83717, t83886)
}
