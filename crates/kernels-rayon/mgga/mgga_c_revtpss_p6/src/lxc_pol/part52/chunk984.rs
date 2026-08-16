//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 984/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk984(t30: f64, t265: f64, t393: f64, t28522: f64, t1469: f64, t2078: f64, t28491: f64, t4186: f64, t45: f64, t606: f64, t7449: f64, t8040: f64, t1113: f64, t1711: f64, t1940: f64, t2071: f64, t2403: f64, t26425: f64, t26585: f64, t27764: f64, t27770: f64, t27773: f64, t27777: f64, t27793: f64, t27800: f64, t27802: f64, t27806: f64, t27810: f64, t27817: f64, t28291: f64, t28456: f64, t28460: f64, t28472: f64, t28490: f64, t33: f64, t7200: f64, t7207: f64, t7428: f64, t7432: f64, t7862: f64, t7869: f64, t8020: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t28523 = piecewise3(t394, 0.0_f64, t28522);
    let t28530 = piecewise3(t120, t28491, t7449 * t1469 / 2.0_f64 + t2078 * t4186 / 2.0_f64 + t28523 * t45 / 2.0_f64 + t8040 * t606 / 2.0_f64);
    let t28577 = 3.0_f64 * t28291 * t27764 + 3.0_f64 / 2.0_f64 * t2403 * t7428 * t7862 - 3.0_f64 / 2.0_f64 * t26425 * t27770 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t27773 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t27777 + 3.0_f64 / 2.0_f64 * t2403 * t8020 * t7200 + t1940 * t28456 * t33 / 2.0_f64 - t1940 * t28460 * t7207 / 2.0_f64 + t1940 * t8020 * t1113 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t26425 * t27793 - t1940 * t26585 * t7869 / 2.0_f64 + t28472 * t27800 - t1940 * t7432 * t27802 / 2.0_f64 - t1940 * t7432 * t27806 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t27810 + t1940 * t7428 * t1711 / 2.0_f64 - t1940 * t7432 * t27817 / 2.0_f64 - t28490;
    (t28530, t28577)
}
