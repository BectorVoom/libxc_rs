//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 925/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk925(t28: f64, t265: f64, t504: f64, t7844: f64, t34030: f64, t1409: f64, t1649: f64, t1877: f64, t2522: f64, t32034: f64, t32047: f64, t33991: f64, t52: f64, t7114: f64, t7649: f64, t7656: f64, t8744: f64, t8748: f64, t8770: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t34052 = t28 * t7844;
    let t34061 = piecewise3(t505, 0.0_f64, t34030);
    let t34066 = piecewise3(t401, 3.0_f64 / 2.0_f64 * t2522 * t8744 * t7649 + t1877 * t33991 * t28 / 2.0_f64 - t1877 * t32034 * t7656 / 2.0_f64 + t1877 * t8744 * t1649 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t2522 * t8748 * t7649 - t1877 * t7114 * t34052 + t1877 * t32047 * t7656 - t1877 * t8748 * t1649 / 2.0_f64, -t8770 * t1409 / 2.0_f64 + t34061 * t52 / 2.0_f64);
    (t34052, t34061, t34066)
}
