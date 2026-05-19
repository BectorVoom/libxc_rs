//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1098/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1098<F: Float>(t2021: F, t6109: F, t899: F, t7305: F, t913: F, t2033: F, t2365: F, t2610: F, t7112: F, t15349: F, t3281: F, t5676: F, t9944: F) -> (F, F, F, F, F) {
    let t28412 = t2021 * t6109 * t899;
    let t28415 = F::cast_from(0.11916829983950142223e0_f64) * t28412 * t913 * t7305;
    let t28419 = F::cast_from(0.29792074959875355558e-1_f64) * t2033 * t2365 * t2610 * t7112;
    let t28421 = F::cast_from(0.29792074959875355558e-1_f64) * t15349 * t3281;
    let t28423 = F::cast_from(0.59584149919750711116e-1_f64) * t5676 * t9944;
    (t28412, t28415, t28419, t28421, t28423)
}
