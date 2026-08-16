//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1999/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1999(t25: f64, t265: f64, t394: f64, t93052: f64, t93099: f64, t12606: f64, t1409: f64, t2064: f64, t2250: f64, t24380: f64, t26807: f64, t3966: f64, t40: f64, t607: f64, t7131: f64, t7865: f64, t92270: f64, t92309: f64, t92349: f64, t93005: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t93100 = t93052 + t93099;
    let t93101 = piecewise3(t395, 0.0_f64, t93100);
    let t93113 = piecewise3(t115, t92270 + t92309 + t92349 + t93005, t93101 * t40 / 2.0_f64 + t26807 * t607 + t7865 * t2250 / 2.0_f64 + t24380 * t1409 / 2.0_f64 + t7131 * t3966 + t2064 * t12606 / 2.0_f64);
    (t93100, t93113)
}
