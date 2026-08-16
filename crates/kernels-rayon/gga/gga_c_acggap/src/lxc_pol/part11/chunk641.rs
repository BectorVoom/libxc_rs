//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 641/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk641(t2959: f64, t2961: f64, t2963: f64, t2966: f64, t1381: f64, t912: f64, t2971: f64, t2710: f64, t2713: f64, t2717: f64, t2737: f64, t2957: f64, t2969: f64, t4061: f64, t4063: f64, t4065: f64, t4069: f64) -> f64 {
    let t5022 = 0.5848223622634646207e0_f64 * t2959;
    let t5023 = 0.34631718211362927518e2_f64 * t2961;
    let t5024 = 0.4883052614935078681e-3_f64 * t2963;
    let t5025 = 0.18311447306006545054e-3_f64 * t2966;
    let t5026 = t1381 * t912;
    let t5027 = 0.11696447245269292414e1_f64 * t5026;
    let t5028 = 48.0_f64 * t2971;
    let t5029 = t4061 - t4063 + t4065 + t4069 - t2957 - t5022 - t5023 + t5024 - t5025 + t2710 - t2713 - t2717 + t2737 + t5027 - t2969 + t5028;
    t5029
}
