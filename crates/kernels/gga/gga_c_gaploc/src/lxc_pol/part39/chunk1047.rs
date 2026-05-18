//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1047/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1047<F: Float>(t43912: F, t32744: F, t9824: F, t10924: F, t1980: F, t13065: F, t2013: F, t43710: F, t825: F, t969: F, t41342: F, t13072: F, t32969: F) -> (F, F, F, F, F, F, F) {
    let t43913 = F::new(0.59584149919750711116e-1) * t43912;
    let t43914 = t32744 * t9824;
    let t43915 = F::new(0.29792074959875355558e-1) * t43914;
    let t43917 = t1980 * t10924 * t9824;
    let t43918 = F::new(0.29792074959875355558e-1) * t43917;
    let t43919 = t2013 * t13065;
    let t43922 = t825 * t969 * t43710;
    let t43924 = F::new(0.29792074959875355558e-1) * t41342;
    let t43925 = t32969 * t13072;
    (t43913, t43915, t43918, t43919, t43922, t43924, t43925)
}
