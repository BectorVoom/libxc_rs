//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1232/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1232(t12520: f64, t1584: f64, t12463: f64, t2207: f64, t3336: f64, t38183: f64, t38190: f64, t40258: f64, t40261: f64, t43688: f64, t43690: f64, t43692: f64, t43695: f64, t43697: f64, t43700: f64) -> f64 {
    let t43702 = t1584 * t12520;
    let t43705 = t2207 * t3336 * t12463;
    let t43707 = -0.54878743191129263322e-1_f64 * t43688 + 0.86682217400542685632e-1_f64 * t43690 + 0.29272321618148349057e-1_f64 * t43692 - 0.16463622957338778997e-1_f64 * t38183 + t38190 + t40258 - 0.12805040077930161442e0_f64 * t43695 - 0.43341108700271342816e-1_f64 * t43697 - 0.43341108700271342816e-1_f64 * t43700 - 0.43341108700271342816e-1_f64 * t43702 - t40261 + 0.65495539973149862688e-2_f64 * t43705;
    t43707
}
