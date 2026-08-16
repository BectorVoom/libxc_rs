//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2573/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2573(t57421: f64, t1235: f64, t371: f64, t5318: f64, t676: f64, t225: f64, t56331: f64, t1789: f64, t2434: f64, t1012: f64, t44958: f64, t13026: f64, t140: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57422 = 0.5081365110289746604e-3_f64 * t57421;
    let t57463 = t1235 * t371 * t676 * t5318;
    let t57464 = 0.14291339372689912324e-3_f64 * t57463;
    let t57465 = t56331 * t225;
    let t57471 = t1235 * t371 * t2434 * t1789;
    let t57480 = t1012 * t44958;
    let t57484 = t140 * t13026;
    (t57422, t57464, t57465, t57471, t57480, t57484)
}
