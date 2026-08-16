//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2752/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2752(t46244: f64, t185: f64, t2658: f64, t55723: f64, t152: f64, t2244: f64, t5499: f64, t4303: f64, t868: f64, t12892: f64, t16693: f64, t16616: f64, t2535: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57996 = 8.0_f64 * t46244;
    let t58005 = 24.0_f64 * t2658 * t185 * t55723;
    let t58008 = 24.0_f64 * t2244 * t152 * t5499;
    let t58009 = t4303 * t868;
    let t58020 = 24.0_f64 * t16693 * t12892;
    let t58021 = t16616 * t2535;
    (t57996, t58005, t58008, t58009, t58020, t58021)
}
