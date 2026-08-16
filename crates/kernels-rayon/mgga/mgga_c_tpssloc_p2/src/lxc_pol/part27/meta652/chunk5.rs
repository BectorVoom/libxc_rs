//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2279/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2279(t25992: f64, t6876: f64, t22592: f64, t7685: f64, t1983: f64, t22948: f64, t5161: f64, t1845: f64, t3914: f64, t26161: f64, t26162: f64, t24994: f64, t6875: f64) -> (f64, f64, f64, f64, f64) {
    let t90428 = 2.0_f64 * t6876 * t25992;
    let t90434 = 6.0_f64 * t7685 * t22592;
    let t90436 = t1983 * t22948 * t5161;
    let t90437 = t1845 * t3914;
    let t90440 = 2.0_f64 * t26161 * t26162 * t90437;
    let t90442 = t6875 * t24994;
    (t90428, t90434, t90436, t90440, t90442)
}
