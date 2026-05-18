//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1071/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1071<F: Float>(t1: F, t23092: F, t22044: F, t739: F, t21460: F, t5654: F, t7802: F, t10912: F, t1422: F, t787: F, t2672: F, t6081: F) -> (F, F, F, F, F, F) {
    let t23348 = t23092 * t1;
    let t23362 = t739 * t22044;
    let t23433 = t739 * t21460;
    let t23469 = t5654 * t7802;
    let t23477 = t787 * t10912 * t1422;
    let t23492 = t6081 * t2672;
    (t23348, t23362, t23433, t23469, t23477, t23492)
}
