//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1142/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1142(t33: f64, t265: f64, t502: f64, t2071: f64, t7862: f64, t8039: f64, t1469: f64, t1711: f64, t1940: f64, t2085: f64, t2403: f64, t57: f64, t7432: f64, t7869: f64, t8020: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t8046 = t2071 * t7862;
    let t8059 = piecewise3(t503, 0.0_f64, t8039);
    let t8064 = piecewise3(t400, 3.0_f64 / 2.0_f64 * t2403 * t8046 + t1940 * t8020 * t33 / 2.0_f64 - t1940 * t7432 * t7869 / 2.0_f64 + t1940 * t2071 * t1711 / 2.0_f64, -t2085 * t1469 / 2.0_f64 + t8059 * t57 / 2.0_f64);
    (t8059, t8064)
}
