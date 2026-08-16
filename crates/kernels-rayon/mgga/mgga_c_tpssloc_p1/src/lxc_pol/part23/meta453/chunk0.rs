//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1305/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1305(t40: f64, t5499: f64, t57973: f64, t46369: f64, t46371: f64, t16637: f64, t20217: f64, t2291: f64, t4104: f64, t5398: f64, t75: f64, t75836: f64, t75847: f64, t75912: f64, t767: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t75950 = 72.0_f64 * t57973 * t5499;
    let t75951 = 16.0_f64 * t46369;
    let t75952 = 0.86748650402413918736e-1_f64 * t46371;
    let t75964 = piecewise3(t146, 0.0_f64, -56.0_f64 / 81.0_f64 * t2291 * t75836 + 16.0_f64 / 9.0_f64 * t16637 * t5398 - 2.0_f64 / 3.0_f64 * t75 * t75847 - 8.0_f64 / 9.0_f64 * t4104 * t20217 + 2.0_f64 / 3.0_f64 * t767 * t75912);
    (t75950, t75951, t75952, t75964)
}
