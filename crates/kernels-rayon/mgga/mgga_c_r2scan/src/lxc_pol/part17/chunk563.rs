//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 563/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk563(t3100: f64, t360: f64, t2124: f64, t2545: f64, t921: f64, t2088: f64, t2095: f64, t2108: f64, t2119: f64, t2122: f64, t2139: f64, t2166: f64, t2606: f64, t2610: f64, t2617: f64, t2621: f64) -> (f64, f64, f64) {
    let t3101 = t360 * t3100;
    let t3105 = t2124 * t2545 * t921;
    let t3108 = t2088 + t2095 + t2108 + t2119 - 0.97574405393827830186e-2_f64 * t2606 - 0.11643651550782197811e-1_f64 * t2610 + 0.12805040077930161442e0_f64 * t2617 + 0.23115257973478049502e0_f64 * t2621 - t2166 + 0.2600466522016280569e0_f64 * t2139 * t3101 + 0.10975748638225852664e0_f64 * t2122 * t3105;
    (t3101, t3105, t3108)
}
