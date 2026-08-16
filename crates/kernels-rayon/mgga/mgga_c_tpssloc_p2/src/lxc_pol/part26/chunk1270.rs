//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1270/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1270(t23153: f64, t2553: f64, t6552: f64, t6637: f64, t117: f64, t4179: f64, t6559: f64, t22893: f64, t23036: f64, t10094: f64, t1888: f64, t22996: f64) -> (f64, f64, f64, f64) {
    let t81637 = t6552 * t6637 * t23153 * t2553;
    let t81640 = t6559 * t4179 * t117;
    let t81642 = t81640 * t22893 * t23036;
    let t81645 = t1888 * t22996 * t10094;
    (t81637, t81640, t81642, t81645)
}
