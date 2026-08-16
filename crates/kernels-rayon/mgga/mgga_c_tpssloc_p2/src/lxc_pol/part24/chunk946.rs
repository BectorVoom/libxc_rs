//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 946/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk946(t10680: f64, t10695: f64, t913: f64, t893: f64, t2840: f64, t891: f64, t275: f64, t2843: f64, t290: f64, t10662: f64, t10524: f64, t2929: f64, t951: f64) -> (f64, f64, f64) {
    let t10696 = t10680 + t10695;
    let t10697 = t10696 * t913;
    let t10699 = 1.0_f64 * t893 * t10697;
    let t10701 = 1.0_f64 / t2840 / t891;
    let t10702 = t275 * t10701;
    let t10704 = 1.0_f64 / t2843 / t290;
    let t10705 = t10662 * t10704;
    let t10707 = 0.51726012919273400301e3_f64 * t10702 * t10705;
    let t10709 = t2929 * t10524 * t951;
    (t10699, t10707, t10709)
}
