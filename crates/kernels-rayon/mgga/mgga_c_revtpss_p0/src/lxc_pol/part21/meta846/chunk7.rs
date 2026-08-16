//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3172/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3172(t3432: f64, t5060: f64, t3436: f64, t12358: f64, t5063: f64, t12226: f64, t1719: f64, t12231: f64, t1733: f64, t45041: f64, t12238: f64, t5105: f64) -> (f64, f64, f64, f64, f64) {
    let t58466 = t5060 * t3432;
    let t58468 = 0.48245938496077605201e2_f64 * t58466 * t3436;
    let t58472 = 1.0_f64 * t5063 * t12358;
    let t58473 = t1719 * t12226;
    let t58475 = 0.51726012919273400301e3_f64 * t58473 * t12231;
    let t58477 = 1.0_f64 * t45041 * t1733;
    let t58479 = 3.0_f64 * t12238 * t5105;
    (t58468, t58472, t58475, t58477, t58479)
}
