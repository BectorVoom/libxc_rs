//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2773/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2773(t10627: f64, t10628: f64, t10632: f64, t14633: f64, t14643: f64, t14653: f64, t14656: f64, t14659: f64, t1553: f64, t18592: f64, t231: f64, t2634: f64, t2642: f64, t4409: f64, t4415: f64, t4417: f64, t4420: f64, t50396: f64, t50914: f64, t73: f64, t830: f64, t833: f64) -> f64 {
    let t50916 = (-360.0_f64 * t10627 * t4415 * t50396 - 36.0_f64 * t2634 * t4417 * t73 + 60.0_f64 * t10628 * t1553 - 36.0_f64 * t10632 * t18592 + 9.0_f64 * t14633 * t833 - 72.0_f64 * t14643 * t14653 - 36.0_f64 * t14643 * t14656 + 9.0_f64 * t14659 * t830 + 9.0_f64 * t2634 * t4420 + 9.0_f64 * t2642 * t4409 + t50914) * t231;
    t50916
}
