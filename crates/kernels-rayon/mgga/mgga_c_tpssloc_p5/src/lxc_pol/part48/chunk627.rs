//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 627/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk627(t28: f64, t265: f64, t504: f64, t2157: f64, t3640: f64, t1254: f64, t1256: f64, t193: f64, t336: f64, t4700: f64, t6834: f64, t7394: f64, t2161: f64, t52: f64, t607: f64, t6855: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t7398 = t2157 * t3640;
    let t7402 = piecewise3(t505, t1256 * t193 * t336 * t7394 - t1254 * t4700 * t7398, t6834);
    let t7407 = piecewise3(t401, t6855, -t2161 * t607 / 2.0_f64 + t7402 * t52 / 2.0_f64);
    (t7398, t7402, t7407)
}
