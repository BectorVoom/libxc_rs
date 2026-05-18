//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 798/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk798<F: Float>(t40088: F, t4782: F, t9272: F, t20700: F, t6710: F, t9438: F, t12535: F, t1407: F, t20551: F, t6914: F, t12531: F, t587: F, t589: F) -> (F, F, F, F, F) {
    let t40353 = t9272 * t4782 * t40088;
    let t40372 = t6710 * t9438 * t20700;
    let t40374 = t1407 * t12535;
    let t40377 = t6914 * t9438 * t20551;
    let t40380 = t587 * t589 * t12531;
    (t40353, t40372, t40374, t40377, t40380)
}
