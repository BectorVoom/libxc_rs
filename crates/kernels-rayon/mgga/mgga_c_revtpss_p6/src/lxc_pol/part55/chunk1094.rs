//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1094/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1094(t33: f64, t265: f64, t502: f64, t25759: f64, t34090: f64, t27799: f64, t34097: f64, t1711: f64, t1962: f64, t34126: f64, t1469: f64, t1940: f64, t2403: f64, t26425: f64, t28460: f64, t28472: f64, t32491: f64, t33888: f64, t34080: f64, t57: f64, t7432: f64, t7862: f64, t7869: f64, t8657: f64, t8677: f64, t8682: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t503 = t265 < t502;
    let t34145 = t25759 * t34090;
    let t34151 = t27799 * t34097;
    let t34153 = t1711 * t1962;
    let t34161 = piecewise3(t503, 0.0_f64, t34126);
    let t34166 = piecewise3(t400, 3.0_f64 / 2.0_f64 * t2403 * t8657 * t7862 + t1940 * t34080 * t33 / 2.0_f64 - t1940 * t32491 * t7869 / 2.0_f64 + t1940 * t8657 * t1711 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t26425 * t34145 - t1940 * t28460 * t8677 / 2.0_f64 + t28472 * t34151 - t1940 * t7432 * t34153 / 2.0_f64 - t1940 * t7432 * t33888 / 2.0_f64, -t8682 * t1469 / 2.0_f64 + t34161 * t57 / 2.0_f64);
    (t34145, t34151, t34153, t34161, t34166)
}
