//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2868/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2868(t41308: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t52039: f64, t52041: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52054: f64, t52057: f64, t52060: f64, t52063: f64, t52091: f64, t52092: f64, t52112: f64) -> f64 {
    let t52114 = t52091 - t52092 - 4.0_f64 / 3.0_f64 * t52039 - 2.0_f64 / 3.0_f64 * t52041 - 4.0_f64 / 3.0_f64 * t52045 + 4.0_f64 / 9.0_f64 * t52047 + 2.0_f64 / 9.0_f64 * t52049 + 10.0_f64 / 27.0_f64 * t52051 - 2.0_f64 / 3.0_f64 * t52054 - 2.0_f64 / 3.0_f64 * t52057 - 10.0_f64 / 9.0_f64 * t52060 - 6.0_f64 * t52063 - 2.0_f64 / 3.0_f64 * t41365 + 2.0_f64 / 9.0_f64 * t41367 + 2.0_f64 / 3.0_f64 * t41308 - 4.0_f64 / 9.0_f64 * t41330 - 8.0_f64 / 27.0_f64 * t41332 + t41334 / 9.0_f64 + 10.0_f64 / 81.0_f64 * t41336 - 6.0_f64 * t52112;
    t52114
}
