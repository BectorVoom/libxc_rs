//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1029/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1029(t28: f64, t265: f64, t504: f64, t123836: f64, t123888: f64, t123938: f64, t1409: f64, t32102: f64, t34061: f64, t3966: f64, t52: f64, t607: f64, t8770: f64, t33853: f64, t532: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t123940 = piecewise3(t505, 0.0_f64, t123836);
    let t123947 = piecewise3(t401, t123888 + t123938, t123940 * t52 / 2.0_f64 - t32102 * t1409 / 2.0_f64 - t34061 * t607 / 2.0_f64 - t8770 * t3966 / 2.0_f64);
    let t123975 = t532 * t33853;
    (t123947, t123975)
}
