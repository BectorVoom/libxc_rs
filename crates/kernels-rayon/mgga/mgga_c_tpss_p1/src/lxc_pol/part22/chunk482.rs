//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 482/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk482(t30: f64, t259: f64, t479: f64, t1692: f64, t1813: f64, t1819: f64, t45: f64, t1812: f64, t33: f64, t1818: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t480 = t259 < t479;
    let t1822 = piecewise3(t120, t1692 * t1813 / 2.0_f64, t1819 * t45 / 2.0_f64);
    let t1823 = t1812 * t33;
    let t1826 = piecewise3(t480, 0.0_f64, t1818);
    (t1822, t1823, t1826)
}
