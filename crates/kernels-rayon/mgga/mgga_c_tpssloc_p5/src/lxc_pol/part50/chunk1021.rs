//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1021/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1021(t1458: f64, t23877: f64, t23880: f64, t26509: f64, t26523: f64, t26533: f64, t26535: f64, t26537: f64, t26539: f64, t26541: f64, t26544: f64, t26547: f64, t26549: f64, t26552: f64, t26554: f64, t4072: f64, t5376: f64, t577: f64, t671: f64, t7010: f64) -> f64 {
    let t26555 = 0.45e1_f64 * t26509 * t577 + 0.135e2_f64 * t26523 * t671 + 0.135e2_f64 * t23877 * t1458 + 27.0_f64 * t23880 * t5376 + 0.135e2_f64 * t7010 * t4072 + t26533 + t26535 + t26537 + t26539 + t26541 + t26544 + t26547 + t26549 + t26552 + t26554;
    t26555
}
