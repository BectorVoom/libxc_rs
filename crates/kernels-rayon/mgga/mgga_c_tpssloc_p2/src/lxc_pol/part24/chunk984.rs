//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 984/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk984(t3241: f64, t52: f64, t9288: f64, t3240: f64, t123: f64) -> (f64, f64, f64) {
    let t11152 = t3241 * t52;
    let t11153 = 1.0_f64 / t11152;
    let t11154 = t11153 * t9288;
    let t11155 = t3240 * t11154;
    let t11156 = t123 * t11155;
    (t11153, t11154, t11156)
}
