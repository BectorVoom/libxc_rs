//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1187/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1187(t2860: f64, t5742: f64, t1977: f64, t237: f64, t1108: f64, t20638: f64, t5500: f64, t1991: f64, t7560: f64, t1083: f64, t5776: f64, t5585: f64) -> (f64, f64, f64, f64, f64) {
    let t20670 = 0.51947577317044391277e2_f64 * t2860 * t5742;
    let t20671 = t237 * t1977;
    let t20674 = 0.10526802520742363173e2_f64 * t20671 * t1108 * t20638;
    let t20676 = 0.10389515463408878255e3_f64 * t2860 * t5500;
    let t20678 = 0.35089341735807877242e1_f64 * t7560 * t1991;
    let t20683 = t5776 * t1083;
    let t20685 = 0.2894756309764656312e3_f64 * t20683 * t5585;
    (t20670, t20674, t20676, t20678, t20685)
}
