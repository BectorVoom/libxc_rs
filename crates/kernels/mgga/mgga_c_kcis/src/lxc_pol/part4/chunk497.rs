//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 497/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk497<F: Float>(t2036: F, t2040: F, t2044: F, t2048: F, t2052: F, t2056: F, t2063: F, t2067: F) -> F {
    let t2128 = F::new(0.9375e-1) * t2036 - F::new(0.9375e-1) * t2040 - F::new(0.25e0) * t2044 + F::new(0.625e-1) * t2048 - F::cast_from(0.101171875e-1_f64) * t2052 + F::cast_from(0.101171875e-1_f64) * t2056 + F::cast_from(0.53958333333333333333e-1_f64) * t2063 - F::cast_from(0.13489583333333333333e-1_f64) * t2067;
    t2128
}
