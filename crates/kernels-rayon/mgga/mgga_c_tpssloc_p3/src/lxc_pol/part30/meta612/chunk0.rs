//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2008/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2008(t23518: f64, t6733: f64, t23669: f64, t995: f64, t3158: f64, t6796: f64, t6802: f64, t23600: f64, t10336: f64, t1920: f64, t1949: f64, t2966: f64, t6805: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t82683 = t6733 * t23518;
    let t82713 = t23669 * t995;
    let t82716 = t6796 * t3158;
    let t82717 = t82716 * t6802;
    let t82736 = t23600 * t995;
    let t82799 = 0.30461741978670859935e-2_f64 * t1920 * t10336 * t1949;
    let t82809 = t1920 * t2966 * t6805;
    (t82683, t82713, t82716, t82717, t82736, t82799, t82809)
}
