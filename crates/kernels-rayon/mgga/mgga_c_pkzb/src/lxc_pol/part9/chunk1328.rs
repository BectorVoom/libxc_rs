//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1328/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1328(t1246: f64, t1255: f64, t1256: f64, t135: f64, t19227: f64, t22390: f64, t22393: f64, t22396: f64, t22398: f64, t22400: f64, t22404: f64, t22406: f64, t22408: f64, t22410: f64, t22478: f64, t23542: f64, t2422: f64, t2430: f64, t2453: f64, t273: f64, t3247: f64, t3254: f64, t3279: f64, t411: f64, t6536: f64, t6547: f64, t6548: f64, t6597: f64, t6598: f64, t8481: f64, t8500: f64, t8504: f64, t938: f64, t952: f64, t957: f64) -> f64 {
    let t23547 = t22390 - t22393 - t22396 + t22398 + t22400 + t22404 + t22406 - t22408 - t22410 + t135 * t273 * (-0.65854491829355115987e0_f64 * t1246 * t6598 - 0.19756347548806534796e1_f64 * t8481 * t952 - 0.39512695097613069591e1_f64 * t1246 * t6548 + 0.39512695097613069591e1_f64 * t411 * t8500 * t2453 - 0.19756347548806534796e1_f64 * t2422 * t3279 + 0.15805078039045227836e2_f64 * t411 * t19227 * t1255 * t6547 - 0.65854491829355115987e0_f64 * t6536 * t1256 + 0.39512695097613069591e1_f64 * t3247 * t2430 + 0.13170898365871023197e1_f64 * t411 * t3254 * t6597 + 0.39512695097613069591e1_f64 * t938 * t8504 + t23542) * t957 - t22478;
    t23547
}
