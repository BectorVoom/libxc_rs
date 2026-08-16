//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1255/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1255(t23000: f64, t33308: f64, t7805: f64, t28279: f64, t3040: f64, t28435: f64, t28811: f64, t7700: f64, t8793: f64, t7703: f64, t13045: f64, t22238: f64, t787: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33310 = t23000 * t33308 * t7805;
    let t33311 = 0.11502877786176224903e1_f64 * t33310;
    let t33313 = 0.71500979903700853338e0_f64 * t28279 * t3040;
    let t33315 = 0.35750489951850426669e0_f64 * t28435 * t3040;
    let t33317 = 0.71500979903700853338e0_f64 * t28811 * t3040;
    let t33319 = 0.21450293971110256002e1_f64 * t8793 * t7700;
    let t33321 = 0.10725146985555128001e1_f64 * t8793 * t7703;
    let t33325 = 0.53625734927775640005e1_f64 * t787 * t22238 * t13045;
    (t33311, t33313, t33315, t33317, t33319, t33321, t33325)
}
