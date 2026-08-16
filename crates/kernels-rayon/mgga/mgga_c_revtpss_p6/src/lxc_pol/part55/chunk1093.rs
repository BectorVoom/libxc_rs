//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1093/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1093(t30: f64, t265: f64, t393: f64, t34126: f64, t1468: f64, t1469: f64, t1940: f64, t2403: f64, t26425: f64, t28460: f64, t28472: f64, t32491: f64, t33740: f64, t34080: f64, t34091: f64, t34098: f64, t34100: f64, t45: f64, t7432: f64, t7749: f64, t7787: f64, t8657: f64, t8660: f64, t8671: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t34127 = piecewise3(t394, 0.0_f64, t34126);
    let t34132 = piecewise3(t120, 3.0_f64 / 2.0_f64 * t2403 * t8657 * t7749 + t1940 * t34080 * t30 / 2.0_f64 - t1940 * t32491 * t7787 / 2.0_f64 + t1940 * t8657 * t1468 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t26425 * t34091 - t1940 * t28460 * t8660 / 2.0_f64 + t28472 * t34098 - t1940 * t7432 * t34100 / 2.0_f64 - t1940 * t7432 * t33740 / 2.0_f64, t8671 * t1469 / 2.0_f64 + t34127 * t45 / 2.0_f64);
    (t34127, t34132)
}
