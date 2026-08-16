//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1226/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1226(t39310: f64, t4190: f64, t8186: f64, t27553: f64, t5897: f64, t4188: f64, t8182: f64, t4189: f64, t6048: f64, t7962: f64, t28450: f64, t4142: f64) -> (f64, f64, f64, f64, f64) {
    let t97989 = 24.0_f64 * t39310 * t8186 * t4190;
    let t97990 = t5897 * t27553;
    let t97991 = t8182 * t4188;
    let t97993 = 2.0_f64 * t97991 * t4190;
    let t97996 = 4.0_f64 * t4189 * t7962 * t6048;
    let t97997 = t4142 * t28450;
    (t97989, t97990, t97993, t97996, t97997)
}
