//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1094/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1094(t1180: f64, t5541: f64, t1648: f64, t583: f64, t14873: f64, t169: f64, t103: f64, t172: f64, t5698: f64, t4048: f64, t561: f64, t1037: f64, t1552: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21053 = t5541 * t1180;
    let t21054 = t1648 * t583;
    let t21072 = t169 * t14873;
    let t21076 = t5698 * t172 * t103;
    let t21084 = t561 * t4048;
    let t21111 = t1037 * t1552;
    (t21053, t21054, t21072, t21076, t21084, t21111)
}
