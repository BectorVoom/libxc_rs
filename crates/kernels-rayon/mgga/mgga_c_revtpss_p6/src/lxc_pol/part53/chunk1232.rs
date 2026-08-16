//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1232/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1232(t29427: f64, t7002: f64, t122950: f64, t125209: f64, t129270: f64, t129431: f64, t129478: f64, t129479: f64, t129480: f64, t129481: f64, t129482: f64, t129483: f64, t129488: f64, t129489: f64, t1518: f64, t32175: f64, t32825: f64, t33643: f64, t33645: f64, t4292: f64, t670: f64, t8563: f64) -> f64 {
    let t129490 = t29427 * t7002;
    let t129491 = t122950 * t1518 + t129270 * t670 + t129431 * t1518 + t32825 * t4292 + t125209 + t129478 + t129479 + t129480 + t129481 + t129482 + t129483 + t129488 + t129489 + t129490 + t32175 + t33643 + t33645 + t8563;
    t129491
}
