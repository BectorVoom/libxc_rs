//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1842/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1842(t22549: f64, t92047: f64, t2031: f64, t90094: f64, t26009: f64, t84219: f64, t90247: f64, t111: f64, t26966: f64, t86588: f64, t86590: f64, t2094: f64, t40611: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t92049 = 160.0_f64 / 9.0_f64 * t22549 * t92047;
    let t92052 = t2031 * t90094;
    let t92056 = 160.0_f64 / 3.0_f64 * t84219 * t26009;
    let t92057 = t2031 * t90247;
    let t92090 = t26966 * t111;
    let t92122 = 8.0_f64 / 3.0_f64 * t86588;
    let t92123 = 4.0_f64 / 3.0_f64 * t86590;
    let t92169 = t2094 * t40611;
    (t92049, t92052, t92056, t92057, t92090, t92122, t92123, t92169)
}
