//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2183/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2183(t26135: f64, t5113: f64, t1983: f64, t23857: f64, t7753: f64, t24991: f64, t6876: f64, t25992: f64, t22592: f64, t7685: f64, t22948: f64, t5161: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90410 = 4.0_f64 * t5113 * t26135;
    let t90418 = 2.0_f64 * t1983 * t7753 * t23857;
    let t90421 = 6.0_f64 * t6876 * t24991;
    let t90428 = 2.0_f64 * t6876 * t25992;
    let t90434 = 6.0_f64 * t7685 * t22592;
    let t90436 = t1983 * t22948 * t5161;
    (t90410, t90418, t90421, t90428, t90434, t90436)
}
