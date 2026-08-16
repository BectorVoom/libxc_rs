//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 575/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk575(t27501: f64, t6035: f64, t1103: f64, t12: f64, t14: f64, t6056: f64, t2247: f64, t6044: f64, t2917: f64, t3746: f64, t17859: f64, t231: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27502 = t6035 * t27501;
    let t27505 = t12 * t1103;
    let t27506 = t27505 * t14;
    let t27507 = t27506 * t6056;
    let t27510 = t6044 * t2247;
    let t27511 = t2917 * t3746;
    let t27512 = t27510 * t27511;
    let t27515 = t231 * t17859;
    (t27502, t27506, t27507, t27511, t27512, t27515)
}
