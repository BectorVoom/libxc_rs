//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 944/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk944(t112778: f64, t112803: f64, t112818: f64, t112820: f64, t112846: f64, t31386: f64, t6579: f64, t23012: f64, t8538: f64, t31339: f64, t81591: f64, t2047: f64, t213: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t114714 = 0.5383034145885385447e-3_f64 * t112778;
    let t114720 = 7.0_f64 / 576.0_f64 * t112803;
    let t114724 = 0.32298204875312312682e-2_f64 * t112818;
    let t114725 = 7.0_f64 / 144.0_f64 * t112820;
    let t114736 = 7.0_f64 / 576.0_f64 * t112846;
    let t114752 = t6579 * t31386;
    let t114759 = t23012 * t8538;
    let t114762 = t81591 * t31339;
    let t114770 = t213 * t2047 * t225;
    (t114714, t114720, t114724, t114725, t114736, t114752, t114759, t114762, t114770)
}
