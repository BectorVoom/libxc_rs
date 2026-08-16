//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1170/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1170(t13765: f64, t4349: f64, t605: f64, t13838: f64, t5552: f64, t42522: f64, t331: f64, t42520: f64, t44221: f64, t44223: f64, t44225: f64, t44228: f64, t44231: f64, t47128: f64, t47153: f64, t47176: f64, t47208: f64, t47224: f64, t47237: f64, t47247: f64, t47249: f64, t47277: f64, t47281: f64, t47293: f64, t47333: f64, t47352: f64, t47354: f64, t47373: f64, t47383: f64, t47394: f64, t47399: f64, t47404: f64, t47425: f64, t47436: f64, t47452: f64, t47458: f64, t47466: f64, t47477: f64, t47482: f64, t47503: f64, t47521: f64, t47547: f64, t47556: f64, t47571: f64, t47578: f64, t47592: f64, t47612: f64, t47620: f64, t47625: f64, t47639: f64, t47656: f64, t47663: f64, t47668: f64, t47699: f64, t47704: f64, t47716: f64, t47725: f64, t47736: f64, t47744: f64, t47767: f64, t47777: f64, t748: f64) -> (f64, f64, f64) {
    let t47784 = t4349 * t13765 * t605;
    let t47785 = 6.0_f64 * t47784;
    let t47786 = t5552 * t13838;
    let t47788 = 2.0_f64 * t42522;
    let t47789 = -t748 * (t47399 + t47394 + t47383 + t47373 + t47354 + t47352 + t47333 + t47293 + t47281 + t47277 + t47247 + t47249 + t47237 + t47224 + t47208 + t47176 + t47153 + t47128 + t47571 + t47556 + t47547 + t47521 + t47578 + t47503 + t47482 + t47404 + t47425 + t47436 + t47452 + t47458 + t47466 + t47477) - t44221 + (t47592 + t47612 + t47620 + t47625 + t47639 + t47656 + t47663 + t47668 + t47699 + t47704 + t47716 + t47725 + t47736 + t47744 + t47767 + t47777) * t331 + t44223 + t44225 + t47785 - t44228 + t42520 + t44231 + 2.0_f64 * t47786 - t47788;
    (t47785, t47788, t47789)
}
