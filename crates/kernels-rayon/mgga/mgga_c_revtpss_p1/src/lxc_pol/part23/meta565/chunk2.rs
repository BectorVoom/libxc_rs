//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2140/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2140(t30: f64, t33: f64, t189: f64, t22789: f64, t512: f64, t1344: f64, t22670: f64, t22769: f64, t5574: f64, t5824: f64, t9605: f64, t1348: f64, t22778: f64, t22783: f64, t5582: f64, t6416: f64, t9617: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t22790 = t22789 * t189;
    let t22791 = t512 * t22790;
    let t22799 = piecewise3(t31, 0.0_f64, 8.0_f64 / 27.0_f64 * t9605 * t22769 - 2.0_f64 / 3.0_f64 * t5574 * t5824 + 2.0_f64 / 3.0_f64 * t1344 * t22670);
    let t22807 = piecewise3(t34, 0.0_f64, 8.0_f64 / 27.0_f64 * t9617 * t22778 - 2.0_f64 / 3.0_f64 * t5582 * t6416 + 2.0_f64 / 3.0_f64 * t1348 * t22783);
    (t22790, t22791, t22799, t22807)
}
