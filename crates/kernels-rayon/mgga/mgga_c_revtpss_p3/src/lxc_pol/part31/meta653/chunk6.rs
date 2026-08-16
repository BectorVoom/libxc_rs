//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2182/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2182(t106554: f64, t27799: f64, t18838: f64, t33: f64, t106482: f64, t106516: f64, t108002: f64, t108005: f64, t108009: f64, t108021: f64, t108028: f64, t108030: f64, t108033: f64, t1711: f64, t1940: f64, t1963: f64, t2403: f64, t27158: f64, t27364: f64, t27368: f64, t27382: f64, t27810: f64, t27817: f64, t29964: f64, t4541: f64, t7091: f64, t7207: f64, t7783: f64, t93404: f64) -> f64 {
    let t108036 = t27799 * t106554;
    let t108043 = t33 * t18838;
    let t108047 = -3.0_f64 * t27158 * t108002 - t1940 * t7091 * t108005 / 2.0_f64 + 3.0_f64 * t4541 * t1963 * t108009 + 3.0_f64 * t2403 * t7783 * t27810 - t1940 * t106516 * t7207 / 2.0_f64 + t1940 * t27364 * t1711 - t1940 * t7091 * t108021 / 2.0_f64 + t1940 * t106482 * t33 / 2.0_f64 + t27382 * t108028 + 3.0_f64 * t27158 * t108030 + 6.0_f64 * t27158 * t108033 + 2.0_f64 * t27382 * t108036 - t1940 * t27368 * t27817 + t1940 * t93404 * t29964 - t1940 * t7091 * t108043 / 2.0_f64;
    t108047
}
