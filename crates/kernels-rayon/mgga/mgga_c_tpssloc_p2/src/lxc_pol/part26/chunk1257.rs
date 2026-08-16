//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1257/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1257(t22480: f64, t4034: f64, t22574: f64, t55246: f64, t8643: f64, t23858: f64, t6876: f64, t26162: f64, t55183: f64, t6535: f64, t9348: f64, t12734: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81412 = 6.0_f64 * t4034 * t22480;
    let t81419 = 9.0_f64 * t22574 * t8643 * t55246;
    let t81422 = 6.0_f64 * t6876 * t23858;
    let t81426 = 18.0_f64 * t22574 * t26162 * t55183;
    let t81430 = 6.0_f64 * t9348 * t6535;
    let t81432 = 12.0_f64 * t12734 * t6535;
    (t81412, t81419, t81422, t81426, t81430, t81432)
}
