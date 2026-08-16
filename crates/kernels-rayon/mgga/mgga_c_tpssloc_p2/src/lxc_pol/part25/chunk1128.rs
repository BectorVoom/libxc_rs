//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1128/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1128(t22986: f64, t22996: f64, t22997: f64, t9627: f64, t252: f64, t2553: f64, t6646: f64, t829: f64, t23020: f64, t6562: f64, t794: f64, t22641: f64, t9523: f64) -> (f64, f64, f64, f64) {
    let t81563 = t22986 * t22996 * t22997 * t9627;
    let t81568 = t22986 * t6646 * t252 * t2553 * t829;
    let t81571 = t6562 * t794 * t23020;
    let t81573 = t22641 * t9523;
    (t81563, t81568, t81571, t81573)
}
