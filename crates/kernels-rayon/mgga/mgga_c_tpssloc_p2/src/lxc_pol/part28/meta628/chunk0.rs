//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1967/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1967(t1408: f64, t1877: f64, t2057: f64, t23302: f64, t24191: f64, t24335: f64, t25021: f64, t25028: f64, t2522: f64, t26563: f64, t26740: f64, t26744: f64, t26756: f64, t47645: f64, t606: f64, t7110: f64, t7545: f64, t7809: f64, t84791: f64, t84797: f64, t86707: f64, t86714: f64, t86727: f64, t86771: f64, t87953: f64, t87978: f64, t87988: f64) -> f64 {
    let t92270 = t1877 * t24335 * t1408 / 2.0_f64 + 6.0_f64 * t26563 * t87978 + t26756 * t86714 - 3.0_f64 * t84797 * t25021 + 2.0_f64 * t26756 * t86771 - 3.0_f64 * t24191 * t86727 - t1877 * t84791 * t7545 / 2.0_f64 + t1877 * t26740 * t606 - t1877 * t26744 * t23302 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t87953 + 3.0_f64 * t2522 * t7110 * t25028 + 3.0_f64 * t47645 * t7809 + 3.0_f64 * t24191 * t87988 - 3.0_f64 * t26563 * t86707;
    t92270
}
