//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1104/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1104(t1936: f64, t34359: f64, t572: f64, t28986: f64, t7553: f64, t7741: f64, t196: f64, t197: f64, t8237: f64, t13272: f64, t8736: f64, t8142: f64, t8435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34360 = t34359 * t1936;
    let t34362 = 6.0_f64 * t572 * t34360;
    let t34363 = t28986 * t1936;
    let t34365 = 6.0_f64 * t572 * t34363;
    let t34366 = t7553 * t7741;
    let t34368 = 6.0_f64 * t572 * t34366;
    let t34399 = t8237 * t196 * t197;
    let t34402 = t13272 * t8736;
    let t34409 = t8435 * t8142;
    (t34360, t34362, t34363, t34365, t34366, t34368, t34399, t34402, t34409)
}
