//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1032/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1032<F: Float>(t3491: F, t5165: F, t639: F, t9099: F, t1676: F, t192: F, t8817: F, t306: F, t9539: F, t2026: F, t3640: F, t5939: F, t154: F, t18086: F, t276: F, t3542: F) -> (F, F, F, F, F, F, F) {
    let t24927 = t3491 * t5165;
    let t24934 = t9099 * t639;
    let t24941 = t9099 * t1676;
    let t24964 = t192 * t8817;
    let t25113 = t306 * t9539;
    let t25189 = t2026 * t5939 * t3640;
    let t25198 = t276 * t154 * t18086 * t3542;
    (t24927, t24934, t24941, t24964, t25113, t25189, t25198)
}
