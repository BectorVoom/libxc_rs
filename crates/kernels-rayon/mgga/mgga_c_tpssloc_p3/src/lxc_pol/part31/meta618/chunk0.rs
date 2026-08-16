//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1867/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1867(t12020: f64, t1385: f64, t1992: f64, t22635: f64, t6439: f64, t28117: f64, t81159: f64, t1377: f64, t6330: f64, t26331: f64, t26332: f64, t5187: f64) -> (f64, f64, f64, f64) {
    let t96910 = t1992 * t22635 * t12020 * t6439 * t1385;
    let t96920 = t81159 * t28117;
    let t96922 = t1377 * t6330;
    let t96925 = t26331 * t22635 * t96922 * t1385;
    let t96929 = t26331 * t22635 * t26332 * t5187;
    (t96910, t96920, t96925, t96929)
}
