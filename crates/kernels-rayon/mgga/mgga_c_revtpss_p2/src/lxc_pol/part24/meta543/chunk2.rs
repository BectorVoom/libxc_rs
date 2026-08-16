//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1605/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1605(t61090: f64, t76947: f64, t76949: f64, t76951: f64, t49897: f64, t18259: f64, t23216: f64, t1469: f64, t4401: f64, t77042: f64, t18263: f64, t5999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t87303 = 24.0_f64 * t61090;
    let t87304 = 144.0_f64 * t76947;
    let t87305 = 48.0_f64 * t76949;
    let t87306 = 4.0_f64 * t76951;
    let t87307 = 0.23392894490538584828e1_f64 * t49897;
    let t87309 = 144.0_f64 * t18259 * t23216;
    let t87312 = 48.0_f64 * t4401 * t77042 * t1469;
    let t87314 = 24.0_f64 * t18263 * t5999;
    (t87303, t87304, t87305, t87306, t87307, t87309, t87312, t87314)
}
