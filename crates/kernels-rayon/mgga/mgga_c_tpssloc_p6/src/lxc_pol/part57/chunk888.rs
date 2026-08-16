//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 888/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk888(t25: f64, t265: f64, t394: f64, t33512: f64, t1408: f64, t1409: f64, t1877: f64, t24191: f64, t2522: f64, t26744: f64, t26756: f64, t31434: f64, t32899: f64, t33466: f64, t33477: f64, t33484: f64, t33486: f64, t40: f64, t7114: f64, t7475: f64, t7545: f64, t8566: f64, t8569: f64, t8580: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t33513 = piecewise3(t395, 0.0_f64, t33512);
    let t33518 = piecewise3(t115, 3.0_f64 / 2.0_f64 * t2522 * t8566 * t7475 + t1877 * t33466 * t25 / 2.0_f64 - t1877 * t31434 * t7545 / 2.0_f64 + t1877 * t8566 * t1408 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t24191 * t33477 - t1877 * t26744 * t8569 / 2.0_f64 + t26756 * t33484 - t1877 * t7114 * t33486 / 2.0_f64 - t1877 * t7114 * t32899 / 2.0_f64, t8580 * t1409 / 2.0_f64 + t33513 * t40 / 2.0_f64);
    (t33513, t33518)
}
