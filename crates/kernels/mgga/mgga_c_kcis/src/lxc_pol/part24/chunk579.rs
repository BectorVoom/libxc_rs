//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 579/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk579<F: Float>(t5084: F, t5087: F, t5089: F, t5092: F, t5094: F, t5097: F, t5100: F, t5166: F, t5170: F, t5173: F, t5178: F, t5183: F, t5186: F) -> F {
    let t5393 = -F::new(0.9375e-1) * t5084 + F::new(0.71944444444444444443e-1) * t5087 + F::new(0.101171875e-1) * t5089 - F::new(0.625e-1) * t5092 + F::new(0.53958333333333333333e-1) * t5094 - F::new(0.53958333333333333333e-1) * t5097 + F::new(0.13489583333333333333e-1) * t5100 + F::new(0.9375e-1) * t5166 - F::new(0.101171875e-1) * t5170 + F::new(0.101171875e-1) * t5173 - F::new(0.20234375e-1) * t5178 - F::new(0.44965277777777777777e-2) * t5183 - F::new(0.16666666666666666667e0) * t5186;
    t5393
}
