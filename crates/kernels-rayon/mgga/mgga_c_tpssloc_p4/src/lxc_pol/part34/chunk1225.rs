//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1225/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1225(t105309: f64, t105311: f64, t105313: f64, t105315: f64, t105317: f64, t105319: f64, t105325: f64, t105329: f64, t105333: f64, t105335: f64, t105337: f64, t105339: f64, t105341: f64, t105345: f64, t105348: f64, t84896: f64, t84897: f64, t98709: f64, t98711: f64, t98725: f64) -> f64 {
    let t108268 = -t105309 / 256.0_f64 + t105311 / 128.0_f64 - t105313 / 64.0_f64 - t105315 / 192.0_f64 - t105317 / 64.0_f64 + 5.0_f64 / 64.0_f64 * t105319 - 7.0_f64 / 8.0_f64 * t98709 - 0.35608770875031824732e0_f64 * t98711 - t84896 - t84897 - 0.12111826828242117256e-2_f64 * t105325 + 0.72670960969452703536e-2_f64 * t105329 + 0.24223653656484234512e-2_f64 * t105333 - t105335 / 768.0_f64 - t105337 / 256.0_f64 - t105339 / 256.0_f64 + 5.0_f64 / 64.0_f64 * t105341 + 0.84782787797694820791e-2_f64 * t98725 + 3.0_f64 / 8.0_f64 * t105345 - 0.24223653656484234512e-2_f64 * t105348;
    t108268
}
