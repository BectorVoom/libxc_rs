//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1024/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1024(t322: f64, t2983: f64, t352: f64, t12601: f64, t10533: f64, t11148: f64, t11162: f64, t12002: f64, t12009: f64, t12622: f64, t12624: f64, t12627: f64, t12629: f64, t12656: f64, t12681: f64, t330: f64, t3413: f64, t3420: f64, t3675: f64, t855: f64) -> (f64, f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t332 = 0.25e1_f64 < t322;
    let t12683 = t352 * t2983;
    let t12692 = piecewise3(t332, t12601, 0.0_f64);
    let t12703 = piecewise5(t323, t12622 * t330 + 2.0_f64 * t12624 * t330 + t12627 * t330 + t12629 * t330, t331, t12656 + t12681, -0.63e1_f64 * t3420 * t12683 - 0.42e1_f64 * t12002 * t3675 - 0.945e1_f64 * t11148 * t12683 - 0.21e1_f64 * t3413 * t10533 - 0.105e1_f64 * t855 * t12692 * t352 - 0.315e1_f64 * t12009 * t3675 - 0.1575e1_f64 * t3420 * t10533 - 0.23625e1_f64 * t11162 * t12683);
    (t12683, t12692, t12703)
}
