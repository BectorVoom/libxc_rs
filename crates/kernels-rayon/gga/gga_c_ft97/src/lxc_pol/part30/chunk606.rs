//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 606/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk606(t1154: f64, t747: f64, t6119: f64, t729: f64, t27819: f64, t681: f64, t6899: f64, t89: f64, t3821: f64, t6008: f64, t193: f64, t1131: f64, t24191: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27820 = t1154 * t747;
    let t27822 = t729 * t6119 * t27820;
    let t27823 = t27819 * t27822;
    let t27825 = t681 * t6899;
    let t27826 = t89 * t27825;
    let t27828 = t6008 * t3821;
    let t27829 = t193 * t27828;
    let t27830 = t89 * t27829;
    let t27832 = t24191 * t1131;
    (t27820, t27823, t27825, t27826, t27829, t27830, t27832)
}
