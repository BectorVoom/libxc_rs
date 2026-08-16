//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1154/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1154(t18245: f64, t423: f64, t14858: f64, t1703: f64, t4869: f64, t4879: f64, t1117: f64, t6021: f64, t3264: f64, t3315: f64, t6020: f64, t3313: f64) -> (f64, f64, f64, f64, f64) {
    let t18247 = 0.621814e-1_f64 * t18245 * t423;
    let t18249 = 0.11696447245269292414e1_f64 * t14858 * t1703;
    let t18251 = 0.11696447245269292414e1_f64 * t4869 * t4879;
    let t18255 = t6021 * t1117;
    let t18257 = 2.0_f64 * t3264 * t18255;
    let t18258 = t6020 * t3315;
    let t18259 = t18258 * t1117;
    let t18261 = 0.16081979498692535067e2_f64 * t3313 * t18259;
    (t18247, t18249, t18251, t18257, t18261)
}
