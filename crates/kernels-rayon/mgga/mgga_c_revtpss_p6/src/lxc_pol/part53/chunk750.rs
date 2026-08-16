//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 750/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk750(t30: f64, t1469: f64, t1996: f64, t45: f64, t7794: f64, t7856: f64, t1544: f64, t33: f64, t1963: f64, t1583: f64, t1711: f64, t1940: f64, t2403: f64, t7091: f64, t7783: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t7861 = piecewise3(t120, t7794, t1996 * t1469 / 2.0_f64 + t7856 * t45 / 2.0_f64);
    let t7862 = t33 * t1544;
    let t7863 = t1963 * t7862;
    let t7869 = t33 * t1583;
    let t7876 = 3.0_f64 / 2.0_f64 * t2403 * t7863 + t1940 * t7783 * t33 / 2.0_f64 - t1940 * t7091 * t7869 / 2.0_f64 + t1940 * t1963 * t1711 / 2.0_f64;
    (t7861, t7862, t7869, t7876)
}
