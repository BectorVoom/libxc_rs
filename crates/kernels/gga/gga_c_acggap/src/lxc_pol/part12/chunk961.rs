//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 961/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk961<F: Float>(t30934: F, t8602: F, t31346: F, t4732: F, t1165: F, t4533: F, t7351: F, t7575: F, t2450: F, t7646: F, t4737: F, t30468: F, t4741: F, t30216: F, t8526: F, t1983: F, t30692: F, t7586: F, t8901: F) -> (F, F, F, F, F, F, F) {
    let t35458 = t30934 * t8602;
    let t35460 = t31346 * t4732;
    let t35464 = t7575 * t1165 * t7351 * t4533;
    let t35466 = t2450 * t7646;
    let t35467 = t35466 * t4737;
    let t35469 = t30468 * t4741;
    let t35471 = t30216 * t8526;
    let t35475 = t30692 * t7586 * t1983 * t8901;
    (t35458, t35460, t35464, t35467, t35469, t35471, t35475)
}
