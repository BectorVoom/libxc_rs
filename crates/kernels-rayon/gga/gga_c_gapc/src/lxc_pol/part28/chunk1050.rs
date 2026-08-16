//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1050/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1050(t11459: f64, t11373: f64, t11382: f64, t11385: f64, t11392: f64, t11403: f64, t11445: f64, t11453: f64, t12086: f64, t12087: f64, t12090: f64, t12093: f64, t12094: f64, t12095: f64, t12096: f64, t12097: f64, t12098: f64, t12099: f64, t12100: f64, t12101: f64) -> f64 {
    let t12104 = 0.10110318318802209383e-5_f64 * t11459;
    let t12105 = -0.90579542097823505425e-7_f64 * t11373 + t12086 + t12087 - 0.4419852458519115466e-8_f64 * t11382 - 0.66297786877786731988e-7_f64 * t11385 + t12090 + 0.57970906942607043474e-5_f64 * t11392 - 0.14340192936791314022e-8_f64 * t11403 + t12093 + t12094 - t12095 - t12096 - t12097 + t12098 - t12099 - t12100 - t12101 - 0.64087860648527174255e-6_f64 * t11445 + 0.98332751566569010434e-8_f64 * t11453 + t12104;
    t12105
}
