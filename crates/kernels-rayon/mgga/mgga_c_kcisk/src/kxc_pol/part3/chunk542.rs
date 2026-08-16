//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 542/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk542(t4224: f64, t4227: f64, t4233: f64, t4238: f64, t4242: f64, t4298: f64, t4302: f64, t4307: f64, t4310: f64, t4314: f64, t4316: f64, t4318: f64) -> f64 {
    let t4564 = 0.20234375e-1_f64 * t4224 - 0.10791666666666666667e0_f64 * t4227 + 0.26979166666666666666e-1_f64 * t4233 - 0.20234375e-1_f64 * t4238 - 0.20833333333333333333e-1_f64 * t4242 + 0.9375e-1_f64 * t4298 - 0.101171875e-1_f64 * t4302 - 0.44965277777777777777e-2_f64 * t4307 - 0.33333333333333333334e0_f64 * t4310 + 0.91666666666666666667e0_f64 * t4314 - 0.5e0_f64 * t4316 + 0.125e0_f64 * t4318;
    t4564
}
