//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 749/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk749(t4402: f64, t4472: f64, t1625: f64, t1628: f64, t1627: f64, t632: f64, t629: f64, t1636: f64, t4246: f64, t4250: f64, t4252: f64, t4258: f64, t4263: f64, t4267: f64, t4271: f64, t4275: f64, t4279: f64, t4282: f64, t4284: f64, t4289: f64, t4295: f64, t4299: f64, t4304: f64, t4308: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4473 = t4402 + t4472;
    let t4475 = t1625 * t1628;
    let t4479 = 1.0_f64 / t1627 / t632;
    let t4480 = t629 * t4479;
    let t4481 = t1636 * t1636;
    let t4500 = 0.9375e-1_f64 * t4246 - 0.1875e0_f64 * t4250 + 0.125e0_f64 * t4252 + 0.1875e0_f64 * t4258 - 0.125e0_f64 * t4263 - 0.9375e-1_f64 * t4267 - 0.20833333333333333333e-1_f64 * t4271 + 0.625e-1_f64 * t4275 - 0.101171875e-1_f64 * t4279 + 0.20234375e-1_f64 * t4282 - 0.26979166666666666666e-1_f64 * t4284 - 0.20234375e-1_f64 * t4289 + 0.26979166666666666666e-1_f64 * t4295 + 0.101171875e-1_f64 * t4299 - 0.44965277777777777777e-2_f64 * t4304 - 0.13489583333333333333e-1_f64 * t4308;
    (t4473, t4475, t4479, t4480, t4481, t4500)
}
