//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 678/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk678<F: Float>(t1406: F, t6575: F, t1339: F, t29984: F, t1381: F, t3141: F, t2754: F, t874: F, t6508: F, t2293: F, t986: F, t2787: F, t6509: F, t123: F, t25760: F, t426: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31356 = t1406 * t6575;
    let t31399 = t1339 * t29984;
    let t31428 = t3141 * t1381;
    let t31585 = t2754 * t874;
    let t31586 = t6508 * t31585;
    let t31590 = t986 * t2293;
    let t31591 = t6508 * t31590;
    let t31769 = t2787 * t6509;
    let t31903 = t25760 * t123;
    let t32005 = t31590 * t426;
    (t31356, t31399, t31428, t31586, t31590, t31591, t31769, t31903, t32005)
}
