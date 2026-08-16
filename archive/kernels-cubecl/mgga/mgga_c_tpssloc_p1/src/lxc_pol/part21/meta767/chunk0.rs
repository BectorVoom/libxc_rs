//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2645/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2645<F: Float>(t112: F, t16506: F, t111: F, t5363: F, t1851: F, t3946: F, t1858: F, t3931: F, t1395: F, t5381: F, t1404: F, t6470: F) -> (F, F, F, F, F, F, F) {
    let t55341 = t16506 * t112;
    let t55353 = t5363 * t111;
    let t55368 = t1851 * t3946;
    let t55374 = t3931 * t1858;
    let t55376 = t1395 * t5381;
    let t55378 = t5363 * t1404;
    let t55388 = t6470 * t111;
    (t55341, t55353, t55368, t55374, t55376, t55378, t55388)
}
