//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2107/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2107(t13829: f64, t2661: f64, t94550: f64, t1873: f64, t94519: f64, t94520: f64, t94527: f64, t94537: f64, t94540: f64, t26004: f64, t5690: f64, t94514: f64, t94523: f64, t94526: f64, t94530: f64, t94534: f64) -> f64 {
    let t98258 = t2661 * t94550 * t13829;
    let t98259 = 0.57165357490759649296e-4_f64 * t98258;
    let t98260 = t94519 * t1873;
    let t98263 = 35.0_f64 / 108.0_f64 * t94520;
    let t98264 = 0.1219527626469539185e-2_f64 * t94527;
    let t98267 = 0.10164000561857065645e-4_f64 * t94537;
    let t98268 = 0.72286371995927450868e-4_f64 * t94540;
    let t98269 = t26004 * t5690;
    let t98270 = 7.0_f64 / 72.0_f64 * t98269;
    let t98271 = -t98259 - 35.0_f64 / 216.0_f64 * t98260 - 7.0_f64 / 48.0_f64 * t94514 - t98263 - t94523 + t94526 - t98264 + 0.57165357490759649296e-4_f64 * t94530 - 0.28582678745379824648e-3_f64 * t94534 + t98267 - t98268 + t98270;
    t98271
}
