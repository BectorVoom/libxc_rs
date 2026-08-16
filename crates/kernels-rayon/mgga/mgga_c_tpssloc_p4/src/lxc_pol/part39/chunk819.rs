//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 819/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk819(t1003: f64, t1058: f64, t1061: f64, t1063: f64, t1610: f64, t1630: f64, t1632: f64, t3180: f64, t3186: f64, t3200: f64, t353: f64, t384: f64, t4615: f64, t4669: f64, t4674: f64, t4678: f64, t4681: f64, t4685: f64, t4689: f64, t4691: f64) -> f64 {
    let t4693 = t1003 * t1632 + t1058 * t4678 + t1058 * t4681 + t1058 * t4689 + t1061 * t4669 + t1063 * t1610 + t1630 * t3180 + 2.0_f64 * t3186 * t4674 - t3200 * t4685 + t353 * t4691 + t384 * t4615;
    t4693
}
