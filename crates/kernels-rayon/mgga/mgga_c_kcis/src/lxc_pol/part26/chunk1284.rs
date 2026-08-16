//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1284/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1284(t1464: f64, t28360: f64, t98409: f64, t28356: f64, t28382: f64, t2038: f64, t28503: f64, t5627: f64, t1615: f64, t27596: f64, t6176: f64, t7497: f64) -> (f64, f64, f64, f64) {
    let t101994 = t1464 * t98409 * t28360;
    let t101997 = t1464 * t28356 * t28382;
    let t102001 = t1464 * t28503 * t2038 * t5627;
    let t102005 = t6176 * t27596 * t7497 * t1615;
    (t101994, t101997, t102001, t102005)
}
