//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1465/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1465(t120944: f64, t120947: f64, t120948: f64, t120954: f64, t120958: f64, t120962: f64, t120964: f64, t120966: f64, t120968: f64, t1849: f64, t2039: f64, t2040: f64, t26878: f64, t27858: f64, t32359: f64, t652: f64, t8690: f64, t96238: f64) -> f64 {
    let t124900 = -2.0_f64 * t2039 * t27858 * t652 + t1849 * t32359 - 2.0_f64 * t2040 * t96238 - t26878 * t8690 + t120944 + t120947 + t120948 - t120954 + t120958 - t120962 - t120964 - t120966 - t120968;
    t124900
}
