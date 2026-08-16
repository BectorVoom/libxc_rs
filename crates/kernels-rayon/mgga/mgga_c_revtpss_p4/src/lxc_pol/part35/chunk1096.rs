//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1096/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1096(t30: f64, t265: f64, t393: f64, t30462: f64, t1469: f64, t2078: f64, t30438: f64, t45: f64, t5825: f64, t8040: f64, t2071: f64, t29939: f64, t1711: f64, t1940: f64, t2403: f64, t26425: f64, t26590: f64, t28460: f64, t29946: f64, t29949: f64, t29953: f64, t29964: f64, t29967: f64, t29970: f64, t30420: f64, t33: f64, t4541: f64, t6416: f64, t7432: f64, t7862: f64, t7869: f64, t8020: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t30463 = piecewise3(t394, 0.0_f64, t30462);
    let t30470 = piecewise3(t120, t30438, t30463 * t45 / 2.0_f64 + t8040 * t1469 + t2078 * t5825 / 2.0_f64);
    let t30471 = t2071 * t29939;
    let t30502 = 3.0_f64 * t4541 * t30471 + 3.0_f64 * t2403 * t8020 * t7862 - 3.0_f64 * t26425 * t29946 + 3.0_f64 * t2403 * t2071 * t29949 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t29953 + t1940 * t30420 * t33 / 2.0_f64 - t1940 * t28460 * t7869 + t1940 * t8020 * t1711 + t1940 * t26590 * t29964 - t1940 * t7432 * t29967 - t1940 * t7432 * t29970 / 2.0_f64 + t1940 * t2071 * t6416 / 2.0_f64;
    (t30463, t30470, t30502)
}
