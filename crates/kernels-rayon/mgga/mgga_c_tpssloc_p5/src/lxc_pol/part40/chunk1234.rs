//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1234/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1234(t28: f64, t12000: f64, t6312: f64, t3711: f64, t5966: f64, t1081: f64, t1302: f64, t18196: f64, t2219: f64, t5178: f64, t19617: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t19618 = t12000 * t6312;
    let t19623 = t3711 * t5966;
    let t19629 = piecewise3(t29, 0.0_f64, 8.0_f64 / 27.0_f64 * t19618 * t1081 + 8.0_f64 / 9.0_f64 * t5178 * t2219 - 2.0_f64 / 9.0_f64 * t19623 * t1081 + 2.0_f64 / 3.0_f64 * t1302 * t18196);
    let t19631 = t19617 / 2.0_f64 + t19629 / 2.0_f64;
    t19631
}
