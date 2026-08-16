//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1217/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1217(t25: f64, t265: f64, t394: f64, t119639: f64, t119676: f64, t119608: f64, t118965: f64, t1409: f64, t30953: f64, t33044: f64, t3966: f64, t40: f64, t607: f64, t8425: f64, t23788: f64, t4255: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t119677 = t119639 + t119676;
    let t119678 = piecewise3(t395, t119608, t119677);
    let t119685 = piecewise3(t115, t118965, t119678 * t40 / 2.0_f64 + t30953 * t1409 / 2.0_f64 + t33044 * t607 / 2.0_f64 + t8425 * t3966 / 2.0_f64);
    let t119691 = t23788 * t4255;
    (t119677, t119685, t119691)
}
