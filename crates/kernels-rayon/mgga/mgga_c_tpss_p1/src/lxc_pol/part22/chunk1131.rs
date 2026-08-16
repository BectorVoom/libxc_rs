//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1131/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1131(t12328: f64, t12421: f64, t12481: f64, t12552: f64, t219: f64, t4294: f64, t1586: f64, t3119: f64, t9739: f64, t1148: f64, t4322: f64, t3118: f64, param_beta: f64) -> (f64, f64, f64, f64, f64) {
    let t12554 = t12328 + t12421 + t12481 + t12552;
    let t12555 = param_beta * t12554;
    let t12557 = t4294 * t219;
    let t12569 = t9739 * t1586 * t3119;
    let t12572 = t4322 * t1148;
    let t12573 = t3118 * t12572;
    (t12554, t12555, t12557, t12569, t12573)
}
