//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 588/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk588<F: Float>(t1386: F, t8232: F, t1882: F, t5953: F, t376: F, t5931: F, t89: F, t23898: F, t23923: F, t1380: F, t1637: F, t5780: F) -> (F, F, F, F, F, F, F) {
    let t24003 = F::new(4.0) / F::new(27.0) * t8232 * t1386;
    let t24004 = t1882 * t5953;
    let t24007 = t89 * t376 * t5931;
    let t24034 = F::new(2.0) / F::new(27.0) * t23898;
    let t24041 = F::new(4.0) / F::new(27.0) * t23923;
    let t24054 = F::new(4.0) / F::new(27.0) * t89 * t1637 * t1380;
    let t24073 = t376 * t5780;
    (t24003, t24004, t24007, t24034, t24041, t24054, t24073)
}
