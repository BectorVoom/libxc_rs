//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1688/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1688<F: Float>(t10296: F, t602: F, t2240: F, t2246: F, t10308: F, t599: F, t90: F, t29: F, t2248: F, t2315: F, t11149: F, t78: F) -> (F, F, F, F, F, F, F) {
    let t45955 = t10296 * t602;
    let t45958 = t2240 * t2246;
    let t45963 = t599 * t10308;
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t45973 = t2248 * t2248;
    let t45979 = t2315 * t2315;
    let t46001 = F::new(1.0) / t78 / t11149;
    (t45955, t45958, t45963, t45972, t45973, t45979, t46001)
}
