//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1961/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1961(t26959: f64, t6486: f64, t1860: f64, t26024: f64, t7031: f64, t2032: f64, t23963: f64, t26016: f64, t84180: f64, t84216: f64, t84242: f64, t84248: f64, t84270: f64, t84280: f64, t84283: f64, t84285: f64, t90072: f64, t90121: f64, t90141: f64) -> f64 {
    let t92031 = 16.0_f64 / 9.0_f64 * t6486 * t26959;
    let t92034 = 16.0_f64 / 9.0_f64 * t1860 * t7031 * t26024;
    let t92039 = 20.0_f64 / 3.0_f64 * t26016 * t84180 + t90121 * t2032 / 3.0_f64 - 880.0_f64 / 27.0_f64 * t84242 - 352.0_f64 / 27.0_f64 * t84248 - 70.0_f64 * t84216 * t90141 - 8.0_f64 / 9.0_f64 * t84270 - t84280 - t92031 - t92034 + 16.0_f64 / 9.0_f64 * t84283 + 176.0_f64 / 27.0_f64 * t84285 + 20.0_f64 * t23963 * t90072;
    t92039
}
