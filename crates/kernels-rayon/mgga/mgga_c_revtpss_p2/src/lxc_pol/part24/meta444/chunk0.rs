//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1403/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1403(t1427: f64, t1903: f64, t22: f64, t9647: f64, t14296: f64, t9303: f64, t5718: f64, t9292: f64, t14099: f64, t2453: f64, t5603: f64, t9692: f64) -> (f64, f64, f64, f64, f64) {
    let t47781 = t9647 * t1427 * t1903 * t22;
    let t47786 = t9303 * t14296;
    let t47802 = t9292 * t5718;
    let t47856 = t2453 * t14099;
    let t47863 = t5603 * t9692;
    (t47781, t47786, t47802, t47856, t47863)
}
