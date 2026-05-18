//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1170/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1170<F: Float>(t13765: F, t4349: F, t605: F, t13838: F, t5552: F, t42522: F, t331: F, t42520: F, t44221: F, t44223: F, t44225: F, t44228: F, t44231: F, t47128: F, t47153: F, t47176: F, t47208: F, t47224: F, t47237: F, t47247: F, t47249: F, t47277: F, t47281: F, t47293: F, t47333: F, t47352: F, t47354: F, t47373: F, t47383: F, t47394: F, t47399: F, t47404: F, t47425: F, t47436: F, t47452: F, t47458: F, t47466: F, t47477: F, t47482: F, t47503: F, t47521: F, t47547: F, t47556: F, t47571: F, t47578: F, t47592: F, t47612: F, t47620: F, t47625: F, t47639: F, t47656: F, t47663: F, t47668: F, t47699: F, t47704: F, t47716: F, t47725: F, t47736: F, t47744: F, t47767: F, t47777: F, t748: F) -> (F, F, F) {
    let t47784 = t4349 * t13765 * t605;
    let t47785 = F::new(6.0) * t47784;
    let t47786 = t5552 * t13838;
    let t47788 = F::new(2.0) * t42522;
    let t47789 = -t748 * (t47399 + t47394 + t47383 + t47373 + t47354 + t47352 + t47333 + t47293 + t47281 + t47277 + t47247 + t47249 + t47237 + t47224 + t47208 + t47176 + t47153 + t47128 + t47571 + t47556 + t47547 + t47521 + t47578 + t47503 + t47482 + t47404 + t47425 + t47436 + t47452 + t47458 + t47466 + t47477) - t44221 + (t47592 + t47612 + t47620 + t47625 + t47639 + t47656 + t47663 + t47668 + t47699 + t47704 + t47716 + t47725 + t47736 + t47744 + t47767 + t47777) * t331 + t44223 + t44225 + t47785 - t44228 + t42520 + t44231 + F::new(2.0) * t47786 - t47788;
    (t47785, t47788, t47789)
}
