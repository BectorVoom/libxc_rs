//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1419/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1419(t28: f64, t265: f64, t504: f64, t121950: f64, t121982: f64, t122012: f64, t122042: f64, t122072: f64, t1409: f64, t31512: f64, t33547: f64, t3966: f64, t52: f64, t607: f64, t8591: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t122075 = piecewise3(t505, 0.0_f64, t121950);
    let t122082 = piecewise3(t401, t121982 + t122012 + t122042 + t122072, t122075 * t52 / 2.0_f64 - t31512 * t1409 / 2.0_f64 - t33547 * t607 / 2.0_f64 - t8591 * t3966 / 2.0_f64);
    t122082
}
