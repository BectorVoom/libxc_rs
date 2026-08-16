//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 611/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk611(t1001: f64, t6539: f64, t286: f64, t285: f64, t2879: f64, t4937: f64, t4959: f64, t6518: f64, t6522: f64, t6526: f64, t6530: f64, t6535: f64, t991: f64) -> (f64, f64, f64) {
    let t6540 = t1001 * t6539;
    let t6541 = t286 * t6540;
    let t6544 = -t2879 + t4937 / 432.0_f64 - t4959 / 144.0_f64 + t991 * t6518 / 216.0_f64 - t991 * t6522 / 144.0_f64 - t991 * t6526 / 144.0_f64 + t991 * t6530 / 288.0_f64 + t285 * t6535 / 48.0_f64 - t285 * t6541 / 96.0_f64;
    (t6540, t6541, t6544)
}
