//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2305/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2305(t2147: f64, t8034: f64, t29624: f64, t7327: f64, t103422: f64, t1653: f64, t18241: f64, t19128: f64, t24858: f64, t27406: f64, t27454: f64, t27462: f64, t27549: f64, t27552: f64, t29720: f64, t3604: f64, t5979: f64, t7283: f64, t7362: f64, t7363: f64, t7373: f64, t7375: f64, t7376: f64, t7377: f64, t94911: f64, t94941: f64, t94947: f64, t95794: f64) -> (f64, f64) {
    let t103683 = t8034 * t2147;
    let t103687 = t29624 * t7327;
    let t103693 = -t94911 - 0.27415567780803773942e-2_f64 * t7283 * t7362 * t24858 * t5979 + 0.14621636149762012769e-1_f64 * t27406 * t27462 - 0.27415567780803773942e-2_f64 * t7283 * t7362 * t7363 * t18241 - 0.54831135561607547884e-2_f64 * t7283 * t7362 * t95794 * t1653 + 0.82246703342411321825e-2_f64 * t7373 * t7375 * t19128 * t7376 - 0.82246703342411321825e-2_f64 * t7283 * t103422 * t27454 - 0.73108180748810063845e-2_f64 * t27549 * t103683 * t27552 - 0.82246703342411321825e-2_f64 * t7373 * t103687 * t7377 - t94941 + 2.0_f64 * t3604 * t29720 - t94947;
    (t103683, t103693)
}
