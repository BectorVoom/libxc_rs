//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1160/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1160<F: Float>(t10208: F, t23213: F, t3185: F, t10204: F, t3206: F, t10088: F, t6475: F, t10093: F, t926: F, t10191: F, t2099: F, t918: F) -> (F, F, F, F, F) {
    let t28231 = t3185 * t23213 * t10208;
    let t28234 = t3206 * t23213 * t10204;
    let t28263 = t3185 * t6475 * t10088;
    let t28266 = t3185 * t926 * t10093;
    let t28283 = t918 * t2099 * t10191;
    (t28231, t28234, t28263, t28266, t28283)
}
