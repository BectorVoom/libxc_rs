//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2399/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2399(t68673: f64, t68693: f64, t894: f64, t901: f64, t59759: f64, t59761: f64, t60308: f64, t60310: f64, t60312: f64, t68638: f64, t68640: f64, t68643: f64, t68646: f64, t68649: f64) -> (f64, f64, f64) {
    let t68694 = t68673 + t68693;
    let t68695 = t894 * t68694;
    let t68697 = t901 * t68694;
    let t68699 = 0.247573125e0_f64 * t68638 + 0.247573125e0_f64 * t68640 - 0.1294625e1_f64 * t68643 + 0.82524375e-1_f64 * t68646 - 0.82785e-1_f64 * t68649 + 0.181155e1_f64 * t59759 - 0.12077e1_f64 * t59761 - 0.33114e0_f64 * t60308 + 0.11038e0_f64 * t60310 + 0.73586666666666666666e-1_f64 * t60312 + 0.258925e1_f64 * t68695 + 0.16504875e0_f64 * t68697;
    (t68695, t68697, t68699)
}
