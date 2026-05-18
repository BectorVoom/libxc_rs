//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 848/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk848<F: Float>(t569: F, t7861: F, t568: F, t1012: F, t4598: F, t2788: F, t4673: F, t2855: F, t4614: F, t2868: F, t1: F, t8025: F) -> (F, F, F, F, F, F) {
    let t8308 = t569 * t7861;
    let t8309 = t568 * t8308;
    let t8312 = t4598 * t1012;
    let t8319 = t4673 * t2788;
    let t8322 = t4614 * t2855;
    let t8327 = t4614 * t2868;
    let t8330 = t8025 * t1;
    (t8309, t8312, t8319, t8322, t8327, t8330)
}
