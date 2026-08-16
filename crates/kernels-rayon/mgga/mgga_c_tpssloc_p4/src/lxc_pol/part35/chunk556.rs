//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 556/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk556(t1147: f64, t1687: f64, t1694: f64, t3403: f64, t300: f64, t3375: f64, t1171: f64, t1706: f64, t1420: f64, t972: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4835 = t1687 * t1147;
    let t4861 = t1694 * t3403;
    let t4869 = t300 * t1687;
    let t4874 = t3375 * t1694;
    let t4887 = t1706 * t1171;
    let t4889 = t1420 * t972;
    (t4835, t4861, t4869, t4874, t4887, t4889)
}
