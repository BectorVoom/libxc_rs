//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 552/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk552<F: Float>(t4246: F, t4250: F, t4252: F, t4258: F, t4263: F, t4267: F, t4271: F, t4275: F, t4279: F, t4282: F, t4284: F, t4289: F, t4295: F, t4299: F, t4304: F, t4308: F) -> F {
    let t4500 = F::cast_from(0.9375e-1_f64) * t4246 - F::cast_from(0.1875e0_f64) * t4250 + F::cast_from(0.125e0_f64) * t4252 + F::cast_from(0.1875e0_f64) * t4258 - F::cast_from(0.125e0_f64) * t4263 - F::cast_from(0.9375e-1_f64) * t4267 - F::cast_from(0.20833333333333333333e-1_f64) * t4271 + F::cast_from(0.625e-1_f64) * t4275 - F::cast_from(0.101171875e-1_f64) * t4279 + F::cast_from(0.20234375e-1_f64) * t4282 - F::cast_from(0.26979166666666666666e-1_f64) * t4284 - F::cast_from(0.20234375e-1_f64) * t4289 + F::cast_from(0.26979166666666666666e-1_f64) * t4295 + F::cast_from(0.101171875e-1_f64) * t4299 - F::cast_from(0.44965277777777777777e-2_f64) * t4304 - F::cast_from(0.13489583333333333333e-1_f64) * t4308;
    t4500
}
