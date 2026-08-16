//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 776/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk776(t2294: f64, t2563: f64, t2133: f64, t259: f64, t547: f64, t6448: f64, t2574: f64, t133: f64, t2526: f64, t1605: f64, t1604: f64, t1610: f64, t2201: f64, t2687: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7457 = t2294 * t2563;
    let t7459 = 0.23115257973478049502e0_f64 * t2133 * t7457;
    let t7460 = t547 * t259;
    let t7461 = t6448 * t7460;
    let t7466 = t2294 * t2574;
    let t7468 = 0.23115257973478049502e0_f64 * t2133 * t7466;
    let t7469 = t133 * t2526;
    let t7470 = t1605 * t7469;
    let t7472 = 0.10975748638225852664e-1_f64 * t1604 * t7470;
    let t7475 = 0.11643651550782197811e-1_f64 * t2201 * t1610 * t2687;
    (t7459, t7460, t7461, t7468, t7470, t7472, t7475)
}
