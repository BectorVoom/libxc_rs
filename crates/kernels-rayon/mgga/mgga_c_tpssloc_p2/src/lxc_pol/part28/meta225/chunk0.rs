//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 989/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk989(t1230: f64, t248: f64, t4733: f64, t3440: f64, t4724: f64, t1193: f64, t1706: f64, t135: f64, t1725: f64, t1174: f64, t1196: f64, t3966: f64) -> (f64, f64, f64, f64, f64) {
    let t5030 = t248 * t1230 * t4733;
    let t5033 = t3440 * t4724;
    let t5036 = t1706 * t1193;
    let t5040 = t135 * t1725;
    let t5041 = t1174 * t5040;
    let t5045 = t1196 * t3966;
    (t5030, t5033, t5036, t5041, t5045)
}
