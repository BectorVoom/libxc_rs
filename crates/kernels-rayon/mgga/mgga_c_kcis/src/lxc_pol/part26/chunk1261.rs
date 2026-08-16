//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1261/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1261(t1370: f64, t7969: f64, t3978: f64, t27636: f64, t27606: f64, t6140: f64, t12844: f64, t27583: f64, t28806: f64, t27555: f64, t18210: f64, t28771: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99198 = t1370 * t7969;
    let t99208 = t3978 * t7969;
    let t99213 = t1370 * t27636;
    let t99219 = t27606 * t6140;
    let t99229 = 0.7722800925925925926e-4_f64 * t27583 * t12844 * t28806;
    let t99233 = t27555 * t6140;
    let t99236 = t18210 * t28771;
    (t99198, t99208, t99213, t99219, t99229, t99233, t99236)
}
