//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 656/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk656(t3793: f64, t3808: f64, t3812: f64, t3814: f64, t418: f64, t4329: f64, t4399: f64, t4472: f64, t4507: f64, t4570: f64, t4613: f64, t4656: f64, t4690: f64, t4767: f64, t4899: f64, t4945: f64, t5008: f64, t5155: f64, t5197: f64, t5225: f64, t5226: f64, t5229: f64, t5232: f64, t5237: f64, t5240: f64, t5243: f64, t5294: f64) -> f64 {
    let t5299 = t4767 + t4945 + t5155 + t3808 + t4613 + 0.17149607247227894789e-2_f64 * t3812 - 7.0_f64 / 288.0_f64 * t3814 - 0.16006300097412701803e-1_f64 * t3793 - t5229 + t4329 + t4399 + t4690 + t5243 - 0.42874018118069736972e-3_f64 * t5226 + t4656 + t5197 + 0.42874018118069736972e-3_f64 * t5240 + t4899 + t4570 + t4507 + t4472 + t5225 - 0.42874018118069736972e-3_f64 * t418 * t5232 - 0.85748036236139473944e-3_f64 * t418 * t5237 + t5008 + t5294;
    t5299
}
