//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1794/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1794(t4135: f64, t198: f64, t40076: f64, t40079: f64, t4147: f64, t47128: f64, t47131: f64, t47134: f64, t47136: f64, t47138: f64, t47140: f64, t47142: f64, t47144: f64, t47146: f64, t47148: f64, t47150: f64, t47152: f64, t532: f64) -> f64 {
    let t47682 = t4135 * t4135;
    let t47687 = -3.0_f64 * t198 * t4147 * t47682 * t532 + t40076 - t40079 + t47128 + t47131 + t47134 - t47136 - t47138 - t47140 + t47142 + t47144 - t47146 - t47148 + t47150 + t47152;
    t47687
}
