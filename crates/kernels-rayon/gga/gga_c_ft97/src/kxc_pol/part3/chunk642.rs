//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 642/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk642(t1602: f64, t7876: f64, t29: f64, t31: f64, t122: f64, t170: f64, t7239: f64, t30: f64, t25: f64, t23: f64, t2999: f64, t26: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7877 = t1602 * t7876;
    let t7905 = 1.0_f64 / t31 / t29;
    let t7906 = t122 * t7905;
    let t7911 = 4.0_f64 * t170 * t7239;
    let t7913 = 1.0_f64 / t30 / t7911;
    let t7914 = t25 * t7913;
    let t7943 = t2999 * t23;
    let t7944 = t26 * t7943;
    (t7877, t7906, t7911, t7914, t7943, t7944)
}
