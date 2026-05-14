//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 803/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk803<F: Float>(t2492: F, t3977: F, t1263: F, t3628: F, t1270: F, t10363: F, t1196: F, t1232: F, t3051: F, t1228: F, t3139: F, t1213: F, t2999: F, t89: F, t1186: F, t3704: F) -> (F, F, F, F, F, F, F, F) {
    let t53923 = t2492 * t3977;
    let t54456 = t3628 * t1263;
    let t54690 = t3628 * t1270;
    let t54859 = t10363 * t1196;
    let t55201 = t3051 * t1232;
    let t55274 = t3139 * t1228;
    let t55558 = t89 * t2999 * t1213;
    let t55562 = t89 * t3704 * t1186;
    (t53923, t54456, t54690, t54859, t55201, t55274, t55558, t55562)
}
