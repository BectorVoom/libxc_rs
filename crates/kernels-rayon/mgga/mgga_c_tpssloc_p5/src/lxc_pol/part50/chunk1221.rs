//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1221/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1221(t28: f64, t265: f64, t504: f64, t119733: f64, t119783: f64, t119677: f64, t1409: f64, t30983: f64, t33074: f64, t3966: f64, t52: f64, t607: f64, t8435: f64, t31222: f64, t7685: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t119784 = t119733 + t119783;
    let t119785 = piecewise3(t505, 0.0_f64, t119677);
    let t119792 = piecewise3(t401, t119784, t119785 * t52 / 2.0_f64 - t30983 * t1409 / 2.0_f64 - t33074 * t607 / 2.0_f64 - t8435 * t3966 / 2.0_f64);
    let t119795 = t7685 * t31222;
    (t119792, t119795)
}
