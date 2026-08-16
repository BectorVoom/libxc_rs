//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1263/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1263(t7284: f64, t94600: f64, t25884: f64, t686: f64, t72: f64, t25895: f64, t7243: f64, t9292: f64, t1032: f64, t4066: f64, t1955: f64, t25878: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94602 = 0.22487184191643109717e-1_f64 * t7284 * t94600;
    let t94604 = t25884 * t72 * t686;
    let t94605 = t25895 * t94604;
    let t94608 = 0.17073386770573548589e-1_f64 * t9292 * t7243;
    let t94609 = t4066 * t1032;
    let t94610 = t1955 * t94609;
    let t94613 = t25878 * t94604;
    (t94602, t94605, t94608, t94609, t94610, t94613)
}
