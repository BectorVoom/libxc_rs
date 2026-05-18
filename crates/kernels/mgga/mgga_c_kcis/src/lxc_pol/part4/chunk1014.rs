//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1014/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1014<F: Float>(t4078: F, t743: F, t4083: F, t733: F, t4086: F, t4096: F, t1431: F, t2466: F, t1438: F, t2471: F, t1452: F, t2475: F) -> (F, F, F, F, F, F, F) {
    let t11977 = t743 * t4078;
    let t11985 = t733 * t4083;
    let t11987 = t733 * t4086;
    let t11995 = t733 * t4096;
    let t12003 = t2466 * t1431;
    let t12005 = t2471 * t1438;
    let t12009 = t2475 * t1452;
    (t11977, t11985, t11987, t11995, t12003, t12005, t12009)
}
