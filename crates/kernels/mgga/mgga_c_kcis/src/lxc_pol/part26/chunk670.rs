//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 670/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk670<F: Float>(t1629: F, t187: F, t2128: F, t4480: F, t6222: F, t633: F, t7268: F, t7270: F, t7273: F, t7398: F, t7533: F, t7537: F, t7566: F) -> F {
    let t7570 = t7268 - t7270 + t7273 - t7398 + t187 * (-t1629 * t7566 - F::new(2.0) * t2128 * t6222 + F::new(2.0) * t4480 * t7537 + t633 * t7533 - t7268 + t7270 - t7273 + t7398);
    t7570
}
