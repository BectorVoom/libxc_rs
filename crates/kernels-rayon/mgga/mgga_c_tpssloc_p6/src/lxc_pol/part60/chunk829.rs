//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 829/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk829(t25: f64, t265: f64, t394: f64, t28755: f64, t1409: f64, t2116: f64, t28469: f64, t40: f64, t5398: f64, t7992: f64, t1760: f64, t8087: f64, t3598: f64, t2154: f64, t6267: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t29507 = piecewise3(t395, 0.0_f64, t28755);
    let t29514 = piecewise3(t115, t28469, t29507 * t40 / 2.0_f64 + t7992 * t1409 + t2116 * t5398 / 2.0_f64);
    let t29531 = t8087 * t1760;
    let t29532 = t3598 * t29531;
    let t29535 = t2154 * t6267;
    (t29514, t29532, t29535)
}
