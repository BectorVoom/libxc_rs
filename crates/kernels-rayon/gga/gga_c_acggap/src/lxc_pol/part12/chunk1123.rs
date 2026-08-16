//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1123/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1123(t2001: f64, t4849: f64, t30811: f64, t4277: f64, t1466: f64, t30540: f64, t4406: f64, t7822: f64, t1470: f64, t1549: f64, t30644: f64, t1554: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35965 = t2001 * t4849;
    let t35967 = t30811 * t4277;
    let t35969 = t30540 * t1466;
    let t35971 = t7822 * t4406;
    let t35973 = t30540 * t1470;
    let t35975 = t30644 * t1549;
    let t35977 = t30644 * t1554;
    (t35965, t35967, t35969, t35971, t35973, t35975, t35977)
}
