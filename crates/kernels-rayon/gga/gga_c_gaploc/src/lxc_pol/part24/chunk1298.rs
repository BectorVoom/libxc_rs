//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1298/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1298(t33310: f64, t28279: f64, t3040: f64, t28435: f64, t28811: f64, t7700: f64, t8793: f64, t7703: f64, t1457: f64, t1966: f64, t1991: f64, t22213: f64, t28570: f64, t313: f64, t32387: f64, t33282: f64, t33284: f64, t33285: f64, t33292: f64, t33297: f64, t33299: f64, t33300: f64, t33304: f64, t590: f64, t5983: f64) -> f64 {
    let t33311 = 0.11502877786176224903e1_f64 * t33310;
    let t33313 = 0.71500979903700853338e0_f64 * t28279 * t3040;
    let t33315 = 0.35750489951850426669e0_f64 * t28435 * t3040;
    let t33317 = 0.71500979903700853338e0_f64 * t28811 * t3040;
    let t33319 = 0.21450293971110256002e1_f64 * t8793 * t7700;
    let t33321 = 0.10725146985555128001e1_f64 * t8793 * t7703;
    let t33322 = t28570 - 0.71500979903700853338e0_f64 * t5983 * t1457 * t32387 - t33282 - t33284 + 0.42900587942220512003e1_f64 * t313 * t33285 * t22213 - t33292 - t33297 - t33299 + 0.2044956050875773316e1_f64 * t1991 * t33300 * t590 - 0.51123901271894332902e1_f64 * t1966 * t33304 * t590 + t33311 + t33313 + t33315 + t33317 + t33319 + t33321;
    t33322
}
