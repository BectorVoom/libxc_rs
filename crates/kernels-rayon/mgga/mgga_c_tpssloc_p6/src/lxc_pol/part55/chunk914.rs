//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 914/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk914(t13351: f64, t232: f64, t815: f64, t23097: f64, t23096: f64, t23106: f64, t23108: f64, t23114: f64, t23119: f64, t25085: f64, t25087: f64, t25089: f64, t25091: f64, t25095: f64) -> (f64, f64) {
    let t25097 = t13351 * t232;
    let t25098 = t815 * t25097;
    let t25099 = t23097 * t25098;
    let t25103 = t23096 - t23106 + t25085 / 768.0_f64 + t25087 / 384.0_f64 - t25089 / 1536.0_f64 + t25091 / 384.0_f64 + 0.40372756094140390854e-3_f64 * t25095 + t23108 + 0.12111826828242117256e-2_f64 * t25099 + 0.33643963411783659045e-4_f64 * t23114 - 7.0_f64 / 2304.0_f64 * t23119;
    (t25097, t25103)
}
