//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1228/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1228(t28063: f64, t7586: f64, t651: f64, t7002: f64, t8233: f64, t122950: f64, t129431: f64, t129436: f64, t129437: f64, t129438: f64, t129440: f64, t129445: f64, t129447: f64, t1519: f64, t1911: f64, t29456: f64, t32825: f64, t32837: f64, t4257: f64, t6985: f64) -> f64 {
    let t129449 = t7586 * t28063;
    let t129452 = t651 * t8233 * t7002;
    let t129454 = -2.0_f64 * t122950 * t1519 - 2.0_f64 * t129431 * t1519 + t1911 * t32837 - 2.0_f64 * t29456 * t6985 - 2.0_f64 * t32825 * t4257 - t129436 - t129437 + 3.0_f64 * t129438 + 3.0_f64 * t129440 - 2.0_f64 * t129445 - 2.0_f64 * t129447 - 2.0_f64 * t129449 - 2.0_f64 * t129452;
    t129454
}
