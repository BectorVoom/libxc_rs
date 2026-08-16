//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 422/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk422(t135: f64, t1725: f64, t1174: f64, t1752: f64, t225: f64, t1243: f64, t5000: f64, t1390: f64, t1845: f64, t172: f64, t1787: f64, t763: f64) -> (f64, f64, f64, f64, f64) {
    let t5040 = t135 * t1725;
    let t5041 = t1174 * t5040;
    let t5055 = t1752 * t225;
    let t5064 = t5000 * t1243;
    let t5122 = t1845 * t1390;
    let t5154 = t1787 * t172;
    let t5155 = t5154 * t763;
    (t5041, t5055, t5064, t5122, t5155)
}
