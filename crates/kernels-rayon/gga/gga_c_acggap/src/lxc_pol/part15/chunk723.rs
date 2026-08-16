//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 723/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk723(t601: f64, t7780: f64, t606: f64, t1973: f64, t1988: f64, t1982: f64, t1983: f64, t361: f64, t1980: f64, t1979: f64, t377: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7781 = t7780 * t601;
    let t7787 = t7780 * t606;
    let t7789 = t1988 * t1973;
    let t7790 = 0.21437009059034868486e-3_f64 * t7789;
    let t7796 = t1982 * t361 * t1983;
    let t7797 = t1980 * t7796;
    let t7798 = 0.14291339372689912324e-3_f64 * t7797;
    let t7799 = t377 * t1979;
    (t7781, t7787, t7790, t7796, t7798, t7799)
}
