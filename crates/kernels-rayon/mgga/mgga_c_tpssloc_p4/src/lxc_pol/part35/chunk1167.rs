//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1167/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1167(t26266: f64, t26361: f64, t26393: f64, t26406: f64, t26429: f64, t26127: f64, t2109: f64, t26012: f64, t33: f64, t7973: f64, t2240: f64, t12571: f64, t7245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27027 = 7.0_f64 / 72.0_f64 * t26266;
    let t27067 = 0.38381794893125283518e-1_f64 * t26361;
    let t27082 = 0.16449340668482264365e-1_f64 * t26393;
    let t27088 = 0.38381794893125283518e-1_f64 * t26406;
    let t27096 = 0.38381794893125283518e-1_f64 * t26429;
    let t27166 = 2.0_f64 / 3.0_f64 * t26127;
    let t27298 = t2109 * t26012;
    let t27331 = t33 * t7973;
    let t27332 = t2240 * t27331;
    let t27341 = t12571 * t7245;
    (t27027, t27067, t27082, t27088, t27096, t27166, t27298, t27331, t27332, t27341)
}
