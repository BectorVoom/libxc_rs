//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1010/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1010(t1518: f64, t27060: f64, t28212: f64, t28214: f64, t28216: f64, t28218: f64, t28221: f64, t28223: f64, t28225: f64, t28227: f64, t28229: f64, t29422: f64, t29427: f64, t29432: f64, t4292: f64, t670: f64, t7586: f64) -> f64 {
    let t29437 = 2.0_f64 * t1518 * t27060 + 2.0_f64 * t1518 * t29432 + 2.0_f64 * t29427 * t670 + 2.0_f64 * t4292 * t7586 + t28212 + t28214 + t28216 + t28218 + t28221 + t28223 + t28225 + t28227 + t28229 + t29422;
    t29437
}
