//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 572/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk572(t4806: f64, t721: f64, t1060: f64, t1072: f64, t495: f64, t3126: f64, t3124: f64, t3143: f64, t503: f64, t1049: f64, t1476: f64, t3237: f64, t532: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4807 = t4806 * t721;
    let t4808 = t1060 * t4807;
    let t4809 = 0.12225e0_f64 * t4808;
    let t4810 = t1072 * t495;
    let t4811 = t4810 * t3126;
    let t4812 = t3124 * t4811;
    let t4814 = t3143 * t503;
    let t4816 = t1049 * t1476;
    let t4817 = 0.1956e1_f64 * t4816;
    let t4843 = t3237 * t532;
    (t4808, t4809, t4810, t4812, t4814, t4816, t4817, t4843)
}
