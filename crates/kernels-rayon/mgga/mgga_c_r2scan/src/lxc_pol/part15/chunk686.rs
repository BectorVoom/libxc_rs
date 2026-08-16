//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 686/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk686(t1616: f64, t481: f64, t2207: f64, t785: f64, t1610: f64, t2201: f64, t2202: f64, t2208: f64, t239: f64, t4715: f64, t5: f64, t1398: f64, t753: f64) -> (f64, f64, f64, f64, f64) {
    let t5181 = t1616 * t481;
    let t5183 = t2207 * t785 * t5181;
    let t5186 = t2201 * t1610 * t2202;
    let t5189 = t2207 * t1610 * t2208;
    let t5193 = 140.0_f64 / 27.0_f64 * t5 * t4715 * t239;
    let t5195 = t5 * t1398 * t753;
    (t5183, t5186, t5189, t5193, t5195)
}
