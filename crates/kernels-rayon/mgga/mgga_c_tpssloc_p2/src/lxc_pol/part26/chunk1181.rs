//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1181/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1181(t28: f64, t2161: f64, t2250: f64, t23820: f64, t24916: f64, t52: f64, t607: f64, t7402: f64, t24562: f64, t111: f64, t7263: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t24923 = piecewise3(t401, t23820, t24916 * t52 / 2.0_f64 - t7402 * t607 - t2161 * t2250 / 2.0_f64);
    let t24924 = t24562 + t24923;
    let t24932 = t7263 * t111;
    (t24924, t24932)
}
