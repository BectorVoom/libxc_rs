//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3170/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3170(t58359: f64, t58372: f64, t58386: f64, t58399: f64, t58413: f64, t58426: f64, t58440: f64, t58453: f64, t1130: f64, t16807: f64, t1151: f64, t16835: f64, t3428: f64) -> (f64, f64, f64) {
    let t58456 = t58359 + t58372 + t58386 + t58399 + t58413 + t58426 + t58440 + t58453;
    let t58460 = t16807 * t1130;
    let t58462 = 3.0_f64 * t58460 * t1151;
    let t58464 = 3.0_f64 * t16835 * t3428;
    (t58456, t58462, t58464)
}
