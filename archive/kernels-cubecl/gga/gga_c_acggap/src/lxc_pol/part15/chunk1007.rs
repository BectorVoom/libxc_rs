//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1007/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1007<F: Float>(t30984: F, t8649: F, t30934: F, t8602: F, t2450: F, t7646: F, t30468: F, t4741: F, t30216: F, t8526: F, t1983: F, t30692: F, t7586: F, t8901: F) -> (F, F, F, F, F, F) {
    let t35456 = t30984 * t8649;
    let t35458 = t30934 * t8602;
    let t35466 = t2450 * t7646;
    let t35469 = t30468 * t4741;
    let t35471 = t30216 * t8526;
    let t35475 = t30692 * t7586 * t1983 * t8901;
    (t35456, t35458, t35466, t35469, t35471, t35475)
}
