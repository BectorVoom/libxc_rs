//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 949/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk949(t10033: f64, t10153: f64, t10155: f64, t10157: f64, t10161: f64, t10163: f64, t10365: f64, t1306: f64, t135: f64, t273: f64, t3282: f64, t3286: f64, t955: f64, t957: f64, t9751: f64, t9753: f64, t9755: f64, t9758: f64, t9759: f64, t9764: f64, t9766: f64, t9768: f64, t9770: f64, t9840: f64, t9842: f64) -> f64 {
    let t10369 = t10365 * t135 * t273 * t957 - 2.0_f64 * t1306 * t3282 * t3286 - t1306 * t955 * t9759 + t10033 + t10153 - t10155 + t10157 - t10161 - t10163 - t9751 - t9753 + t9755 + t9758 - t9764 + t9766 - t9768 + t9770 + t9840 + t9842;
    t10369
}
