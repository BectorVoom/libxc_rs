//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 383/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk383(t2339: f64, t2342: f64, t56: f64, t649: f64, t66: f64, t45: f64, t5: f64, t103: f64, t681: f64, t52: f64, t672: f64, t678: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2344 = 0.16081824322151104822e2_f64 * t2339 * t2342;
    let t2346 = t649 * t66 * t56;
    let t2349 = t45 * t5;
    let t2350 = t103 * t681;
    let t2353 = t672 * t52;
    let t2354 = 1.0_f64 / t2353;
    let t2355 = t678 * t678;
    (t2344, t2346, t2349, t2350, t2354, t2355)
}
