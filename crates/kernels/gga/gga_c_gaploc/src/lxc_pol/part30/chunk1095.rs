//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1095/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1095<F: Float>(t28409: F, t2021: F, t6109: F, t899: F, t7305: F, t913: F, t2033: F, t2365: F, t2610: F, t7112: F, t15349: F, t3281: F) -> (F, F, F, F, F) {
    let t28410 = F::new(0.30674340763136599741e1) * t28409;
    let t28412 = t2021 * t6109 * t899;
    let t28415 = F::new(0.11916829983950142223e0) * t28412 * t913 * t7305;
    let t28419 = F::new(0.29792074959875355558e-1) * t2033 * t2365 * t2610 * t7112;
    let t28421 = F::new(0.29792074959875355558e-1) * t15349 * t3281;
    (t28410, t28412, t28415, t28419, t28421)
}
