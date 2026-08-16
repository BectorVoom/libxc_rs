//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1231/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1231(t25: f64, t265: f64, t394: f64, t191: f64, t192: f64, t8107: f64, t2020: f64, t7688: f64, t8690: f64, t33043: f64, t1409: f64, t32907: f64, t40: f64, t8678: f64, t33079: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t33746 = t8107 * t191 * t192;
    let t33747 = t33746 * t2020;
    let t33748 = t8690 * t7688;
    let t33750 = piecewise3(t395, 0.0_f64, t33043);
    let t33755 = piecewise3(t115, t32907, t8678 * t1409 / 2.0_f64 + t33750 * t40 / 2.0_f64);
    let t33756 = t33755 + t33079;
    (t33746, t33747, t33748, t33750, t33756)
}
