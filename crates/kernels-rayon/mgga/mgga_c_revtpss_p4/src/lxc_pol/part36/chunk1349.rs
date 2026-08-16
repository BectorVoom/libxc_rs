//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1349/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1349(t30: f64, t265: f64, t393: f64, t1916: f64, t30191: f64, t30194: f64, t114401: f64, t117: f64, t572: f64, t114089: f64, t113492: f64, t1469: f64, t2129: f64, t22671: f64, t30727: f64, t45: f64, t5825: f64, t8161: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t114877 = 18.0_f64 * t1916 * t30191;
    let t114879 = 9.0_f64 * t1916 * t30194;
    let t114882 = 3.0_f64 * t572 * t117 * t114401;
    let t116053 = piecewise3(t394, 0.0_f64, t114089);
    let t116063 = piecewise3(t120, t113492, t116053 * t45 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t30727 * t1469 + 3.0_f64 / 2.0_f64 * t8161 * t5825 + t2129 * t22671 / 2.0_f64);
    (t114877, t114879, t114882, t116063)
}
