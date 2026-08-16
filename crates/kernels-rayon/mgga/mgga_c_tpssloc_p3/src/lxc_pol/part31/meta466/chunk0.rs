//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1623/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1623(t25374: f64, t25927: f64, t1081: f64, t1530: f64, t28: f64, t4303: f64, t1649: f64, t776: f64, t868: f64, t1307: f64, t1845: f64, t645: f64, t72: f64, t7431: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25928 = t25927 * t25374;
    let t25930 = t1081 * t1530;
    let t25934 = t28 * t4303;
    let t25938 = t1649 * t776;
    let t25945 = t1649 * t868;
    let t25988 = t1845 * t1307;
    let t26009 = t72 * t7431 * t645;
    (t25928, t25930, t25934, t25938, t25945, t25988, t26009)
}
