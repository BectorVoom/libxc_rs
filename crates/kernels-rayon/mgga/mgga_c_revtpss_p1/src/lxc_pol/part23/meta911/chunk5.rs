//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2931/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2931(t41361: f64, t51978: f64, t52573: f64, t63320: f64, t77515: f64, t77518: f64, t77521: f64, t77527: f64, t77531: f64, t77535: f64, t77736: f64, t77739: f64) -> f64 {
    let t77923 = 0.61977e1_f64 * t77515 - 0.17215833333333333333e1_f64 * t77518 - 0.929655e1_f64 * t77521 + 0.41678e0_f64 * t77736 - 0.187551e1_f64 * t77739 - t52573 + 0.16068111111111111111e1_f64 * t51978 + 0.20839e0_f64 * t63320 + 0.5356037037037037037e0_f64 * t41361 - 0.103295e1_f64 * t77527 - 0.103295e1_f64 * t77531 + 0.123954e2_f64 * t77535;
    t77923
}
