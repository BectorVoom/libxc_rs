//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1372/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1372(t25: f64, t265: f64, t394: f64, t119677: f64, t118965: f64, t1409: f64, t31823: f64, t33750: f64, t3966: f64, t40: f64, t607: f64, t8678: f64, t24932: f64, t7467: f64, t27888: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t123037 = piecewise3(t395, 0.0_f64, t119677);
    let t123044 = piecewise3(t115, t118965, t123037 * t40 / 2.0_f64 + t31823 * t1409 / 2.0_f64 + t33750 * t607 / 2.0_f64 + t8678 * t3966 / 2.0_f64);
    let t123050 = t24932 * t7467;
    let t123052 = t27888 * t7467;
    (t123044, t123050, t123052)
}
