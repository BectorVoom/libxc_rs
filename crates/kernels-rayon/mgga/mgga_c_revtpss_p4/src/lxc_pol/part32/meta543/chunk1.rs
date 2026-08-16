//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1855/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1855(t26519: f64, t93160: f64, t25372: f64, t95536: f64, t7398: f64, t822: f64, t93170: f64, t95746: f64, t7064: f64, t95575: f64, t2067: f64, t41117: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95813 = t93160 * t26519;
    let t95822 = t25372 * t95536;
    let t95825 = t822 * t7398;
    let t95836 = t93170 * t95746;
    let t95859 = t7064 * t95575;
    let t95862 = 0.81814717454467823679e-4_f64 * t41117 * t2067;
    (t95813, t95822, t95825, t95836, t95859, t95862)
}
