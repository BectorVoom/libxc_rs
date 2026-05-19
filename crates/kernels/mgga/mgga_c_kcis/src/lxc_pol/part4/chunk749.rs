//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 749/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk749<F: Float>(t4402: F, t4472: F, t1625: F, t1628: F, t1627: F, t632: F, t629: F, t1636: F, t4246: F, t4250: F, t4252: F, t4258: F, t4263: F, t4267: F, t4271: F, t4275: F, t4279: F, t4282: F, t4284: F, t4289: F, t4295: F, t4299: F, t4304: F, t4308: F) -> (F, F, F, F, F, F) {
    let t4473 = t4402 + t4472;
    let t4475 = t1625 * t1628;
    let t4479 = F::new(1.0) / t1627 / t632;
    let t4480 = t629 * t4479;
    let t4481 = t1636 * t1636;
    let t4500 = F::new(0.9375e-1) * t4246 - F::new(0.1875e0) * t4250 + F::new(0.125e0) * t4252 + F::new(0.1875e0) * t4258 - F::new(0.125e0) * t4263 - F::new(0.9375e-1) * t4267 - F::cast_from(0.20833333333333333333e-1_f64) * t4271 + F::new(0.625e-1) * t4275 - F::cast_from(0.101171875e-1_f64) * t4279 + F::new(0.20234375e-1) * t4282 - F::cast_from(0.26979166666666666666e-1_f64) * t4284 - F::new(0.20234375e-1) * t4289 + F::cast_from(0.26979166666666666666e-1_f64) * t4295 + F::cast_from(0.101171875e-1_f64) * t4299 - F::cast_from(0.44965277777777777777e-2_f64) * t4304 - F::cast_from(0.13489583333333333333e-1_f64) * t4308;
    (t4473, t4475, t4479, t4480, t4481, t4500)
}
