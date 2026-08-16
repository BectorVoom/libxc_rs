//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 924/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk924(t25: f64, t265: f64, t394: f64, t34030: f64, t1408: f64, t1409: f64, t1877: f64, t2522: f64, t32034: f64, t32047: f64, t33991: f64, t34004: f64, t40: f64, t7114: f64, t7475: f64, t7545: f64, t8744: f64, t8748: f64, t8760: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t34031 = piecewise3(t395, 0.0_f64, t34030);
    let t34036 = piecewise3(t115, 3.0_f64 / 2.0_f64 * t2522 * t8744 * t7475 + t1877 * t33991 * t25 / 2.0_f64 - t1877 * t32034 * t7545 / 2.0_f64 + t1877 * t8744 * t1408 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t2522 * t8748 * t7475 - t1877 * t7114 * t34004 + t1877 * t32047 * t7545 - t1877 * t8748 * t1408 / 2.0_f64, t8760 * t1409 / 2.0_f64 + t34031 * t40 / 2.0_f64);
    (t34031, t34036)
}
