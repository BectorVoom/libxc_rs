//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1101/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1101(t30570: f64, t508: f64, t1518: f64, t8065: f64, t29494: f64, t7488: f64, t2107: f64, t22483: f64, t26161: f64, t29498: f64, t2051: f64, t5883: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30571 = t508 * t30570;
    let t30578 = t8065 * t1518;
    let t30581 = t7488 * t29494;
    let t30584 = t2107 * t22483;
    let t30586 = t26161 * t29498;
    let t30589 = t2051 * t5883;
    (t30571, t30578, t30581, t30584, t30586, t30589)
}
