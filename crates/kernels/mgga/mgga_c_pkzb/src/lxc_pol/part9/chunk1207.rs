//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1207/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1207<F: Float>(t1246: F, t1255: F, t1256: F, t135: F, t19227: F, t22390: F, t22393: F, t22396: F, t22398: F, t22400: F, t22404: F, t22406: F, t22408: F, t22410: F, t22478: F, t23542: F, t2422: F, t2430: F, t2453: F, t273: F, t3247: F, t3254: F, t3279: F, t411: F, t6536: F, t6547: F, t6548: F, t6597: F, t6598: F, t8481: F, t8500: F, t8504: F, t938: F, t952: F, t957: F) -> (F,) {
    let t23547 = t22390 - t22393 - t22396 + t22398 + t22400 + t22404 + t22406 - t22408 - t22410 + t135 * t273 * (-0.65854491829355115987e0 * t1246 * t6598 - 0.19756347548806534796e1 * t8481 * t952 - 0.39512695097613069591e1 * t1246 * t6548 + 0.39512695097613069591e1 * t411 * t8500 * t2453 - 0.19756347548806534796e1 * t2422 * t3279 + 0.15805078039045227836e2 * t411 * t19227 * t1255 * t6547 - 0.65854491829355115987e0 * t6536 * t1256 + 0.39512695097613069591e1 * t3247 * t2430 + 0.13170898365871023197e1 * t411 * t3254 * t6597 + 0.39512695097613069591e1 * t938 * t8504 + t23542) * t957 - t22478;
    (t23547,)
}
