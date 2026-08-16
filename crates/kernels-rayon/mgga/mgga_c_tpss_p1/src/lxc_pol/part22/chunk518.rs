//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 518/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk518(t2148: f64, t762: f64, t124: f64, t2133: f64, t227: f64, t767: f64) -> (f64, f64, f64) {
    let t2149 = t762 * t2148;
    let t2153 = t762 * t124 * t2133;
    let t2157 = 1.0_f64 / t767 / t227;
    (t2149, t2153, t2157)
}
