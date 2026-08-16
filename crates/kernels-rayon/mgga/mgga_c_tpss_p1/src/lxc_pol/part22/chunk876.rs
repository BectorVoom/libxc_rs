//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 876/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk876(t2101: f64, t600: f64, t629: f64, t98: f64, t99: f64, t2: f64, t22: f64, t106: f64, t107: f64, t10: f64, t555: f64, t551: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7592 = t600 * t2101;
    let t7593 = t629 * t629;
    let t7594 = 1.0_f64 / t7593;
    let t7612 = t99 * t98;
    let t7613 = 1.0_f64 / t7612;
    let t7622 = t2 * t22;
    let t7628 = t107 * t106;
    let t7629 = 1.0_f64 / t7628;
    let t7651 = t10 * t555;
    let t7653 = t551 * t22;
    (t7592, t7594, t7613, t7622, t7629, t7651, t7653)
}
