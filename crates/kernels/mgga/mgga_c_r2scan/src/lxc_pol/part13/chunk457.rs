//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 457/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk457<F: Float>(t11: F, t1643: F, t1645: F, t2040: F, t2073: F, t2074: F, t5: F, param_eta: F) -> F {
    let t2078 = t1643 - F::new(10.0) / F::new(3.0) * t1645 + F::new(5.0) * t5 * t11 * t2040 - F::new(45.0) * param_eta * (t2073 + t2074);
    t2078
}
