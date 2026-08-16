//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 786/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk786(t7922: f64, t7928: f64, t2019: f64, t2323: f64, t7926: f64, t7487: f64, t8346: f64, t2145: f64, t27: f64, t3118: f64, t570: f64, t2046: f64, t7297: f64, t8482: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38172 = 0.19863479950205658386e-3_f64 * t7922;
    let t38174 = 0.487802396665200453e-2_f64 * t7928;
    let t38312 = t2019 * t7926 * t2323;
    let t38314 = t7487 * t8346;
    let t38318 = t2145 * t27 * t3118 * t570;
    let t38322 = t2046 * t7297 * t8482;
    (t38172, t38174, t38312, t38314, t38318, t38322)
}
