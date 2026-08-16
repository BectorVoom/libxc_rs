//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3166/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3166(t56183: f64, t56185: f64, t56187: f64, t56189: f64, t56194: f64, t56198: f64, t56203: f64, t56207: f64, t56209: f64, t56212: f64, t56214: f64, t56216: f64) -> f64 {
    let t58399 = 0.80513333333333333334e0_f64 * t56183 - 0.12077e1_f64 * t56185 - 0.60385e0_f64 * t56187 - 0.181155e1_f64 * t56189 - 0.60384999999999999999e0_f64 * t56194 - 0.60384999999999999999e0_f64 * t56198 - 0.36230999999999999999e1_f64 * t56203 - 0.20128333333333333333e0_f64 * t56207 + 0.40256666666666666667e0_f64 * t56209 + 0.20128333333333333333e0_f64 * t56212 + 0.12077e1_f64 * t56214 - 0.33547222222222222222e0_f64 * t56216;
    t58399
}
