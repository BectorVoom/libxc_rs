//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 944/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk944(t33092: f64, t464: f64, t14575: f64, t7942: f64, t8306: f64, t8111: f64, t880: f64, t32194: f64, t7963: f64, t2176: f64, t3912: f64, t2132: f64, t2217: f64, t7885: f64, t864: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33093 = t33092 * t464;
    let t33097 = t7942 * t8306 * t14575;
    let t33100 = 0.19756347548806534796e1_f64 * t8111 * t880;
    let t33104 = t7963 * t8306 * t32194;
    let t33107 = 0.65854491829355115987e0_f64 * t2176 * t3912;
    let t33118 = t7885 * t2132 * t2217 * t864;
    (t33093, t33097, t33100, t33104, t33107, t33118)
}
