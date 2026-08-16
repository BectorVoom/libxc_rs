//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1065/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1065(t2021: f64, t7517: f64, t1: f64, t21794: f64, t787: f64, t10929: f64, t1984: f64, t6110: f64, t6134: f64, t5514: f64, t935: f64, t1858: f64, t2530: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23183 = t2021 * t7517;
    let t23203 = t787 * t21794 * t1;
    let t23220 = t1984 * t10929;
    let t23279 = t6134 * t6110;
    let t23292 = t5514 * t935;
    let t23296 = t1858 * t2530;
    (t23183, t23203, t23220, t23279, t23292, t23296)
}
