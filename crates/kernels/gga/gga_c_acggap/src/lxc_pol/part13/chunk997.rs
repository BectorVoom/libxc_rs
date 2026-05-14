//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 997/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk997<F: Float>(t30984: F, t8649: F, t30934: F, t8602: F, t31346: F, t4732: F, t1165: F, t4533: F, t7351: F, t7575: F, t2450: F, t7646: F, t4737: F, t30468: F, t4741: F, t30216: F, t8526: F) -> (F, F, F, F, F, F, F) {
    let t35456 = t30984 * t8649;
    let t35458 = t30934 * t8602;
    let t35459 = 0.22642626448514989489e-1 * t35458;
    let t35460 = t31346 * t4732;
    let t35464 = t7575 * t1165 * t7351 * t4533;
    let t35466 = t2450 * t7646;
    let t35467 = t35466 * t4737;
    let t35469 = t30468 * t4741;
    let t35471 = t30216 * t8526;
    (t35456, t35459, t35460, t35464, t35467, t35469, t35471)
}
