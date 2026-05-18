//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 930/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk930<F: Float>(t41337: F, t13077: F, t28439: F, t32744: F, t9824: F, t10924: F, t1980: F, t41342: F, t13072: F, t32969: F, t10867: F, t41511: F) -> (F, F, F, F, F, F, F) {
    let t43910 = F::new(0.11916829983950142223e0) * t41337;
    let t43912 = t13077 * t28439;
    let t43913 = F::new(0.59584149919750711116e-1) * t43912;
    let t43914 = t32744 * t9824;
    let t43915 = F::new(0.29792074959875355558e-1) * t43914;
    let t43917 = t1980 * t10924 * t9824;
    let t43918 = F::new(0.29792074959875355558e-1) * t43917;
    let t43924 = F::new(0.29792074959875355558e-1) * t41342;
    let t43925 = t32969 * t13072;
    let t43927 = t10867 * t41511;
    (t43910, t43913, t43915, t43918, t43924, t43925, t43927)
}
