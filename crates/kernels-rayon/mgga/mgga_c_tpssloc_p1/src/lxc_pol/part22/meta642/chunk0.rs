//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2182/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2182(t12283: f64, t19976: f64, t19886: f64, t19815: f64, t3802: f64, t20000: f64, t54566: f64, t16398: f64, t19873: f64, t16397: f64, t5234: f64, t5252: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56837 = t12283 * t19976;
    let t56853 = t12283 * t19886;
    let t56878 = t19815 * t3802;
    let t56883 = t54566 * t20000;
    let t56885 = t16398 * t19873;
    let t56888 = t5234 * t16397 * t5252;
    (t56837, t56853, t56878, t56883, t56885, t56888)
}
