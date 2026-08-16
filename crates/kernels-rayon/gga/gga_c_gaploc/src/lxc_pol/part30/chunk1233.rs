//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1233/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1233(t10686: f64, t10688: f64, t13063: f64, t161: f64, t1841: f64, t1843: f64, t1850: f64, t1854: f64, t1858: f64, t29434: f64, t32471: f64, t32474: f64, t32477: f64, t32480: f64, t32483: f64, t32485: f64, t32488: f64, t32490: f64, t32493: f64, t32504: f64, t5227: f64, t734: f64) -> f64 {
    let t32508 = t32471 + t32474 - t32477 + t32480 - t32483 - t32485 - t32488 - t32490 + t32493 - 0.17090058289204942853e-2_f64 * t1841 * t1858 * t13063 * t734 + 0.17090058289204942853e-2_f64 * t1850 * t10686 * t161 * t1854 - 0.17090058289204942853e-2_f64 * t5227 * t10688 + 0.17090058289204942853e-2_f64 * t1841 * t1843 * t32504 - t29434;
    t32508
}
