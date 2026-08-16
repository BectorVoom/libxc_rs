//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1173/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1173(t142: f64, t4586: f64, t7436: f64, t7440: f64, t8803: f64, t2030: f64, t5183: f64, t7815: f64, t31693: f64, t31700: f64, t31708: f64, t4680: f64, t7564: f64, t8449: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36063 = t7436 * t142 * t4586;
    let t36065 = t7440 * t8803;
    let t36066 = 11.0_f64 / 288.0_f64 * t36065;
    let t36068 = t2030 * t7815 * t5183;
    let t36070 = 0.14291339372689912324e-2_f64 * t31693;
    let t36072 = 0.28582678745379824648e-3_f64 * t31700;
    let t36075 = 0.57165357490759649296e-3_f64 * t31708;
    let t36077 = t7564 * t4680 * t8449;
    (t36063, t36066, t36068, t36070, t36072, t36075, t36077)
}
