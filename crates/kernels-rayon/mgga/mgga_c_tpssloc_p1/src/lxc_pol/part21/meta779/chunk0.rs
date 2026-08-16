//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2702/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2702(t5303: f64, t53945: f64, t16336: f64, t5310: f64, t5286: f64, t3792: f64, t1827: f64, t54124: f64, t16288: f64, t5289: f64, t19805: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56906 = t53945 * t5303;
    let t56909 = t16336 * t5310;
    let t56913 = t5286 * t5286;
    let t56914 = t56913 * t3792;
    let t56919 = t54124 * t1827;
    let t56921 = t16288 * t5289;
    let t56923 = t19805 * t68;
    (t56906, t56909, t56913, t56914, t56919, t56921, t56923)
}
