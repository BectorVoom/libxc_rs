//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 991/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk991(t7433: f64, t8970: f64, t1181: f64, t22040: f64, t604: f64, t7493: f64, t21118: f64, t7351: f64, t7426: f64, t1165: f64, t21955: f64, t30806: f64) -> (f64, f64, f64, f64) {
    let t35092 = t7433 * t8970;
    let t35093 = 0.18868855373762491241e-2_f64 * t35092;
    let t35096 = t7493 * t1181 * t604 * t22040;
    let t35097 = 0.21437009059034868486e-2_f64 * t35096;
    let t35100 = t7426 * t1181 * t7351 * t21118;
    let t35101 = 0.12862205435420921092e-2_f64 * t35100;
    let t35113 = t30806 * t1165 * t604 * t21955;
    (t35093, t35097, t35101, t35113)
}
