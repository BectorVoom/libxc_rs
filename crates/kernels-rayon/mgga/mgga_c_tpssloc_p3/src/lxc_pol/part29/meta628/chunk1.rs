//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2073/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2073(t7288: f64, t85660: f64, t225: f64, t24758: f64, t24637: f64, t7294: f64, t2121: f64, t3427: f64, t7295: f64, t24901: f64, t3640: f64, t11947: f64, t7394: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t86473 = t85660 * t7288;
    let t86475 = t24758 * t225;
    let t86494 = t7294 * t24637;
    let t86501 = t2121 * t3427 * t7295;
    let t86513 = t24901 * t3640;
    let t86517 = t7394 * t11947;
    (t86473, t86475, t86494, t86501, t86513, t86517)
}
