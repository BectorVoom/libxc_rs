//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2243/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2243(t16662: f64, t1894: f64, t236: f64, t6591: f64, t5568: f64, t81956: f64, t28389: f64, t81963: f64, t81764: f64, t81789: f64, t81808: f64, t87234: f64, t87248: f64, t87256: f64, t87263: f64, t87271: f64, t87273: f64, t92597: f64, t98690: f64, t98694: f64, t98696: f64, t98701: f64, t98703: f64) -> f64 {
    let t98707 = t6591 * t1894 * t236 * t16662;
    let t98709 = t81956 * t5568;
    let t98711 = t81963 * t28389;
    let t98713 = -7.0_f64 / 2304.0_f64 * t98690 - t87234 - 119.0_f64 / 1728.0_f64 * t81764 - t92597 + t87248 + t87256 + t87263 - 0.31625325607076639503e-2_f64 * t81789 + 7.0_f64 / 144.0_f64 * t98694 + 0.84782787797694820792e-2_f64 * t98696 - 119.0_f64 / 6912.0_f64 * t81808 - t87271 + t87273 + 0.40372756094140390854e-3_f64 * t98701 - t98703 / 48.0_f64 - 0.12111826828242117256e-2_f64 * t98707 - 7.0_f64 / 48.0_f64 * t98709 - 0.59347951458386374554e-1_f64 * t98711;
    t98713
}
