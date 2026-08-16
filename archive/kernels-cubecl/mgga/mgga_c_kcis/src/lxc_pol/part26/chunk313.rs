//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 313/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk313<F: Float>(t1530: F, t1536: F, t1540: F, t1544: F, t1549: F, t1553: F) -> F {
    let t1636 = F::cast_from(0.9375e-1_f64) * t1530 - F::cast_from(0.9375e-1_f64) * t1536 + F::cast_from(0.625e-1_f64) * t1540 - F::cast_from(0.101171875e-1_f64) * t1544 + F::cast_from(0.101171875e-1_f64) * t1549 - F::cast_from(0.13489583333333333333e-1_f64) * t1553;
    t1636
}
