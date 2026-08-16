//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1873/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1873(t25305: f64, t95793: f64, t25410: f64, t7419: f64, t93240: f64, t26519: f64, t93160: f64, t25372: f64, t95536: f64, t92840: f64, t7398: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95808 = t25305 * t95793;
    let t95811 = t93240 * t25410 * t7419;
    let t95813 = t93160 * t26519;
    let t95822 = t25372 * t95536;
    let t95823 = t95822 * t92840;
    let t95825 = t822 * t7398;
    (t95808, t95811, t95813, t95822, t95823, t95825)
}
