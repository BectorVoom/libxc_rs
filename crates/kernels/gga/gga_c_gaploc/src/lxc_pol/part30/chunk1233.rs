//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1233/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1233<F: Float>(t10686: F, t10688: F, t13063: F, t161: F, t1841: F, t1843: F, t1850: F, t1854: F, t1858: F, t29434: F, t32471: F, t32474: F, t32477: F, t32480: F, t32483: F, t32485: F, t32488: F, t32490: F, t32493: F, t32504: F, t5227: F, t734: F) -> F {
    let t32508 = t32471 + t32474 - t32477 + t32480 - t32483 - t32485 - t32488 - t32490 + t32493 - F::cast_from(0.17090058289204942853e-2_f64) * t1841 * t1858 * t13063 * t734 + F::cast_from(0.17090058289204942853e-2_f64) * t1850 * t10686 * t161 * t1854 - F::cast_from(0.17090058289204942853e-2_f64) * t5227 * t10688 + F::cast_from(0.17090058289204942853e-2_f64) * t1841 * t1843 * t32504 - t29434;
    t32508
}
