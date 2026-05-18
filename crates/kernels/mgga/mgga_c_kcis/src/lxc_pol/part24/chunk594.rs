//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 594/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk594<F: Float>(t2943: F, t6320: F, t6338: F, t932: F, t3088: F, t4612: F, t6328: F, t6332: F, t6336: F, t1036: F, t1670: F, t245: F, t3078: F, t4654: F) -> (F, F, F, F) {
    let t6341 = t2943 * t6320;
    let t6343 = t932 * t6338;
    let t6349 = -F::new(0.991e-2) * t6341 + F::new(0.1982e-1) * t6343 + t3088 + F::new(0.27516666666666666666e-2) * t4612 - F::new(0.27516666666666666667e-2) * t6328 + F::new(0.8255e-2) * t6332 - F::new(0.41275e-2) * t6336;
    let t6352 = -t3078 * t6320 / F::new(8.0) + t4654 * t1670 / F::new(2.0) + t1036 * t6338 / F::new(4.0) + t245 * t6349 / F::new(2.0);
    (t6341, t6343, t6349, t6352)
}
