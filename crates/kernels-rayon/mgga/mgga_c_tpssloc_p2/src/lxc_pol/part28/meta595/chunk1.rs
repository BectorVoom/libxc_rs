//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1892/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1892(t1649: f64, t2749: f64, t23788: f64, t57893: f64, t2752: f64, t13487: f64, t1390: f64, t16018: f64, t26062: f64, t645: f64, t72: f64, t26066: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t89982 = t1649 * t2749;
    let t89987 = t23788 * t57893;
    let t89992 = t2752 * t1649;
    let t89993 = t89992 * t13487;
    let t90023 = t1390 * t16018;
    let t90072 = t72 * t26062 * t645;
    let t90076 = t72 * t26066 * t645;
    (t89982, t89987, t89993, t90023, t90072, t90076)
}
