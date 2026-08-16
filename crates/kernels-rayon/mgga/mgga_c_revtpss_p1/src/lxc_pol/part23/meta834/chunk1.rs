//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2705/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2705(t20349: f64, t698: f64, t20352: f64, t20343: f64, t20346: f64, t20273: f64, t2439: f64, t6467: f64, t6464: f64, t6461: f64, t20567: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t68538 = t698 * t20349;
    let t68540 = t698 * t20352;
    let t68548 = t698 * t20343;
    let t68550 = t698 * t20346;
    let t68567 = t698 * t20273;
    let t68583 = t2439 * t6467;
    let t68585 = t2439 * t6464;
    let t68590 = t2439 * t6461;
    let t68609 = t300 * t20567;
    (t68538, t68540, t68548, t68550, t68567, t68583, t68585, t68590, t68609)
}
