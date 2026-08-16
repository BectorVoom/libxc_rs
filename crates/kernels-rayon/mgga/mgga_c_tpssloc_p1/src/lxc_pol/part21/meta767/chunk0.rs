//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2645/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2645(t112: f64, t16506: f64, t111: f64, t5363: f64, t1851: f64, t3946: f64, t1858: f64, t3931: f64, t1395: f64, t5381: f64, t1404: f64, t6470: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t55341 = t16506 * t112;
    let t55353 = t5363 * t111;
    let t55368 = t1851 * t3946;
    let t55374 = t3931 * t1858;
    let t55376 = t1395 * t5381;
    let t55378 = t5363 * t1404;
    let t55388 = t6470 * t111;
    (t55341, t55353, t55368, t55374, t55376, t55378, t55388)
}
