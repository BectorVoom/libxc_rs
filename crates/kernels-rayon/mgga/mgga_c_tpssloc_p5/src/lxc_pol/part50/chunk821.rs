//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 821/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk821(t25: f64, t265: f64, t394: f64, t202: f64, t8365: f64, t8369: f64, t193: f64, t2752: f64, t870: f64, t1070: f64, t3216: f64, t336: f64, t8409: f64, t8413: f64, t40: f64, t8374: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t8418 = t202 * t8365;
    let t8421 = t202 * t8369;
    let t8424 = -t193 * t2752 * t8421 + t193 * t8418 * t870;
    let t8425 = piecewise3(t395, t1070 * t193 * t336 * t8409 - t193 * t3216 * t336 * t8413, t8424);
    let t8428 = piecewise3(t115, t8374, t8425 * t40 / 2.0_f64);
    (t8418, t8421, t8424, t8425, t8428)
}
