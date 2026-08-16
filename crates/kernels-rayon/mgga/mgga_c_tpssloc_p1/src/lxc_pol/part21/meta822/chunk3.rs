//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2892/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2892(t10820: f64, t14344: f64, t17355: f64, t17366: f64, t2900: f64, t2925: f64, t2933: f64, t42020: f64, t42123: f64, t4449: f64, t5762: f64, t5775: f64, t5791: f64, t60033: f64, t60035: f64, t60037: f64, t60039: f64, t60041: f64, t60044: f64, t60047: f64, t60050: f64, t60053: f64, t60056: f64, t60332: f64, t60338: f64, t60343: f64, t943: f64, t951: f64, t952: f64) -> f64 {
    let t60346 = -t60033 + t60035 + t60037 - t60039 - t60041 - t60044 - t60047 + t60050 + t60053 + t60056 + 0.11696447245269292414e1_f64 * t4449 * t14344 - 0.11696447245269292414e1_f64 * t42020 * t5775 + 0.5848223622634646207e0_f64 * t10820 * t5791 + 0.11696447245269292414e1_f64 * t2900 * t17366 + 0.5848223622634646207e0_f64 * t943 * t60332 * t951 + 0.32163958997385070134e2_f64 * t42123 * t5762 + 0.11696447245269292414e1_f64 * t60338 * t952 + 0.5848223622634646207e0_f64 * t17355 * t2925 + 0.17315859105681463759e2_f64 * t60343 * t2933;
    t60346
}
