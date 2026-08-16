//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 925/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk925(t2202: f64, t862: f64, t2522: f64, t673: f64, t2516: f64, t235: f64, t2697: f64, t2519: f64, t262: f64, t265: f64, t5543: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8627 = t2202 * t862;
    let t8629 = t673 * t2522;
    let t8631 = t673 * t2516;
    let t8633 = t235 * t2697;
    let t8647 = t673 * t2519;
    let t8660 = t262 * t5543 * t265;
    let t8661 = 0.93011851851851851854e0_f64 * t8660;
    let t8662 = t599 * t235;
    (t8627, t8629, t8631, t8633, t8647, t8660, t8661, t8662)
}
