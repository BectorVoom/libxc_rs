//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2720/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2720(t14383: f64, t2398: f64, t40108: f64, t14616: f64, t2619: f64, t162: f64, t40207: f64, t4403: f64, t40119: f64, t40121: f64, t14386: f64, t2615: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t50045 = 12.0_f64 * t2398 * t14383;
    let t50046 = 0.5848223622634646207e0_f64 * t40108;
    let t50047 = t14616 * t2619;
    let t50048 = 0.73245789224026180216e-3_f64 * t50047;
    let t50051 = 36.0_f64 * t40207 * t162 * t4403;
    let t50055 = 4.0_f64 * t40119;
    let t50056 = 0.31168546390226634765e3_f64 * t40121;
    let t50058 = t14386 * t2615;
    (t50045, t50046, t50048, t50051, t50055, t50056, t50058)
}
