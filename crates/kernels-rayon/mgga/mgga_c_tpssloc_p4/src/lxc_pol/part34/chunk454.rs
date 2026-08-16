//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 454/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk454(t2822: f64, t1008: f64, t191: f64, t349: f64, t1011: f64, t68: f64) -> (f64, f64, f64, f64) {
    let t3003 = 5.0_f64 / 18.0_f64 * t2822;
    let t3030 = 1.0_f64 / t1008 / t191;
    let t3031 = t349 * t3030;
    let t3032 = t1011 * t68;
    (t3003, t3030, t3031, t3032)
}
