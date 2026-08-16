//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 50/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk50(t40: f64, t148: f64, t74: f64, t52: f64, t77: f64, t145: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let cbrt2 = (M_CBRT2 as f64);
    let t146 = t40 <= zeta_threshold;
    let t149 = piecewise3(t146, t148, t74);
    let t150 = t52 <= zeta_threshold;
    let t151 = piecewise3(t150, t148, t77);
    let t152 = t149 + t151 - 2.0_f64;
    let t153 = t145 * t152;
    let t154 = cbrt2;
    (t152, t153, t154)
}
