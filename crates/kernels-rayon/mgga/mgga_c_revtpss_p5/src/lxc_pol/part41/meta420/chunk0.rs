//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1475/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1475(t2178: f64, t6765: f64, t6934: f64, t5891: f64, t8259: f64, t1504: f64, t1513: f64, t8268: f64, t5915: f64, t31058: f64, t5895: f64, t5823: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31518 = t6765 * t2178;
    let t31533 = t2178 * t6934;
    let t31538 = t8259 * t5891;
    let t31541 = t1513 * t1504;
    let t31542 = t8268 * t31541;
    let t31545 = t8259 * t5915;
    let t31548 = t31058 * t5895;
    let t31551 = t8268 * t5823;
    (t31518, t31533, t31538, t31541, t31542, t31545, t31548, t31551)
}
