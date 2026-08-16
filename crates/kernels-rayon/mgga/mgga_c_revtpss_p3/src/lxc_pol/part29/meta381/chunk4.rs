//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1366/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1366(t2777: f64, t5759: f64, t2439: f64, t1398: f64, t1892: f64, t4086: f64, t543: f64, t2782: f64, t5659: f64, t72: f64, t686: f64, t4101: f64) -> (f64, f64, f64) {
    let t14202 = t2777 * t5759;
    let t14203 = t2439 * t14202;
    let t14207 = t4086 * t1892 * t1398 * t543;
    let t14209 = 0.10975748638225852664e-1_f64 * t2782 * t14207;
    let t14215 = t5659 * t72;
    let t14216 = t14215 * t686;
    let t14218 = 0.19514881078765566038e-1_f64 * t4101 * t14216;
    (t14203, t14209, t14218)
}
