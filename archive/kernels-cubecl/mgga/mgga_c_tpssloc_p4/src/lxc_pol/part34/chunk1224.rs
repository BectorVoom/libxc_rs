//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1224/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1224<F: Float>(t105278: F, t105282: F, t105286: F, t105288: F, t105290: F, t105292: F, t105294: F, t105296: F, t105299: F, t105304: F, t84857: F, t84859: F, t87213: F, t87243: F, t98618: F, t98647: F, t98690: F, t98694: F, t98696: F) -> F {
    let t108249 = -F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t98618 + F::cast_from(0.12111826828242117256e-2_f64) * t98647 - t84857 + F::cast_from(0.72670960969452703536e-2_f64) * t105278 + F::cast_from(0.72670960969452703536e-2_f64) * t105282 - F::cast_from(0.14534192193890540707e-1_f64) * t105286 + t105288 / F::cast_from(64.0_f64) + t105290 / F::cast_from(32.0_f64) + t105292 / F::cast_from(64.0_f64) + t105294 / F::cast_from(128.0_f64) - t105296 / F::cast_from(256.0_f64) + t84859 + t105299 / F::cast_from(768.0_f64) + F::cast_from(0.10093189023535097713e-3_f64) * t87213 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t98690 - F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t87243 - t105304 / F::cast_from(32.0_f64) + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t98694 + F::cast_from(0.50869672678616892474e-1_f64) * t98696;
    t108249
}
