//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 700/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk700(t23143: f64, t242: f64, t6612: f64, t812: f64, t117: f64, t229: f64, t67: f64, t6559: f64) -> (f64, f64, f64) {
    let t23144 = 35.0_f64 / 432.0_f64 * t23143;
    let t23145 = t6612 * t242;
    let t23146 = t812 * t23145;
    let t23163 = t229 * t67 * t117;
    let t23164 = t6559 * t23163;
    (t23144, t23146, t23164)
}
