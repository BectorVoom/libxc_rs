//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2413/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2413(t2439: f64, t2440: f64, t2829: f64, t10977: f64, t2465: f64, t686: f64, t72: f64, t11061: f64, t11064: f64, t2410: f64, t2832: f64, t775: f64) -> (f64, f64, f64, f64, f64) {
    let t41125 = t2439 * t2440 * t2829;
    let t41129 = t2465 * t10977 * t72 * t686;
    let t41137 = t11061 * t11064;
    let t41153 = t2410 * t2410;
    let t41154 = 1.0_f64 / t41153;
    let t41161 = t775 * t2832;
    (t41125, t41129, t41137, t41154, t41161)
}
