//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3139/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3139(t1196: f64, t12548: f64, t5197: f64, t16643: f64, t3531: f64, t16682: f64, t1732: f64, t3433: f64, t12411: f64, t12556: f64, t1756: f64, t43752: f64) -> (f64, f64, f64, f64, f64) {
    let t57849 = 0.11696447245269292414e1_f64 * t1196 * t5197 * t12548;
    let t57851 = 0.31168546390226634765e3_f64 * t3531 * t16643;
    let t57853 = 0.35089341735807877242e1_f64 * t3531 * t16682;
    let t57854 = t3433 * t1732;
    let t57856 = 18.0_f64 * t57854 * t12411;
    let t57860 = 0.12304822629859687989e5_f64 * t1196 * t43752 * t1756 * t12556;
    (t57849, t57851, t57853, t57856, t57860)
}
