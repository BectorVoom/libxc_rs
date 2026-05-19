//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 116/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk116<F: Float>(t321: F, t322: F, t326: F, t334: F, t31: F, t4: F, t79: F) -> (F, F, F) {
    let t337 = F::new(1.0) + F::cast_from(0.13900948042322754167e-2_f64) * t321 * t322 - F::cast_from(0.57970906942607043474e-5_f64) * t326 * t334;
    let t338 = F::new(1.0) / t337;
    let t344 = F::cast_from(0.11073577833333333333e-2_f64) * t4 * t79 * t31;
    (t337, t338, t344)
}
