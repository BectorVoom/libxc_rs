//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1769/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1769(t6928: f64, t80766: f64, t22804: f64, t22808: f64, t22715: f64, t547: f64, t1329: f64, t22822: f64, t281: f64, t6924: f64, t22794: f64, t120: f64, t22816: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t80767 = t80766 * t6928;
    let t80769 = t22804 * t22808;
    let t80775 = t22715 * t547;
    let t80776 = t80775 * t1329;
    let t80779 = t22822 * t6924 * t281;
    let t80780 = t80779 * t22794;
    let t80782 = t22816 * t120;
    (t80767, t80769, t80775, t80776, t80779, t80780, t80782)
}
