//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 508/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk508<F: Float>(t1577: F, t2937: F, t2936: F, t1475: F, t2902: F) -> (F, F, F) {
    let t2938 = t2937 * t1577;
    let t2939 = t2936 * t2938;
    let t2941 = t2902 * t1475;
    (t2938, t2939, t2941)
}
