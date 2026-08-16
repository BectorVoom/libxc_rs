//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1093/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1093<F: Float>(t17972: F, t2068: F, t2263: F, t30984: F, t8649: F, t30934: F, t8602: F, t31346: F, t4732: F, t1165: F, t4533: F, t7351: F, t7575: F) -> (F, F, F, F, F) {
    let t35454 = t2068 * t17972 * t2263;
    let t35456 = t30984 * t8649;
    let t35458 = t30934 * t8602;
    let t35460 = t31346 * t4732;
    let t35464 = t7575 * t1165 * t7351 * t4533;
    (t35454, t35456, t35458, t35460, t35464)
}
