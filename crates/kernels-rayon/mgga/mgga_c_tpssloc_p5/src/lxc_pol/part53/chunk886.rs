//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 886/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk886(t25: f64, t265: f64, t394: f64, t32071: f64, t1877: f64, t2522: f64, t32030: f64, t32034: f64, t32044: f64, t32047: f64, t40: f64, t606: f64, t607: f64, t6542: f64, t6671: f64, t7114: f64, t8744: f64, t8748: f64, t8760: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t32072 = piecewise3(t395, 0.0_f64, t32071);
    let t32077 = piecewise3(t115, 3.0_f64 / 2.0_f64 * t2522 * t8744 * t6542 + t1877 * t32030 * t25 / 2.0_f64 - t1877 * t32034 * t6671 / 2.0_f64 + t1877 * t8744 * t606 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t2522 * t8748 * t6542 - t1877 * t7114 * t32044 + t1877 * t32047 * t6671 - t1877 * t8748 * t606 / 2.0_f64, t32072 * t40 / 2.0_f64 + t8760 * t607 / 2.0_f64);
    (t32072, t32077)
}
