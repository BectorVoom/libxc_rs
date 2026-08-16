//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 543/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk543(t4245: f64, t4308: f64, t312: f64, t4239: f64, t1218: f64, t1253: f64, t301: f64, t317: f64, t4027: f64, t4135: f64, t4182: f64, t4247: f64, t4251: f64, t4300: f64, t830: f64, t880: f64) -> (f64, f64, f64) {
    let t4309 = t4245 + t4308;
    let t4311 = t4239 * t312;
    let t4317 = -t1218 * t880 - t1253 * t830 - t301 * t4309 - t317 * t4027 - t317 * t4135 + 4.0_f64 * t4182 - 2.0_f64 * t4247 - 2.0_f64 * t4251 - 2.0_f64 * t4300 + 2.0_f64 * t4311;
    (t4309, t4311, t4317)
}
