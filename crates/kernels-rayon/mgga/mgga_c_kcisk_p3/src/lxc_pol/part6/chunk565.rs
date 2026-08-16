//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 565/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk565(t3564: f64, t7869: f64, t1428: f64, t7764: f64, t457: f64, t2191: f64) -> (f64, f64, f64, f64) {
    let t7870 = t3564 * t7869;
    let t7873 = t1428 * t7764;
    let t7874 = t457 * t7873;
    let t7877 = t2191 * t2191;
    (t7870, t7873, t7874, t7877)
}
