//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1177/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1177(t24527: f64, t10731: f64, t639: f64, t16554: f64, t16571: f64, t16582: f64, t24534: f64, t24536: f64, t24539: f64, t24542: f64, t16584: f64, t24600: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28942 = 12.0_f64 * t24527;
    let t28943 = t10731 * t639;
    let t28950 = 0.5848223622634646207e0_f64 * t16554;
    let t28951 = 120.0_f64 * t16571;
    let t28952 = 0.48159733137676571078e0_f64 * t16582;
    let t28954 = 60.0_f64 * t24534;
    let t28955 = 0.51947577317044391276e2_f64 * t24536;
    let t28956 = 0.17544670867903938621e1_f64 * t24539;
    let t28957 = 0.17544670867903938621e1_f64 * t24542;
    let t28958 = 12.0_f64 * t16584;
    let t28959 = 3.0_f64 * t24600;
    (t28942, t28943, t28950, t28951, t28952, t28954, t28955, t28956, t28957, t28958, t28959)
}
