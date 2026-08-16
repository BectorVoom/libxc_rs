//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 64/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk64(t129: f64, t173: f64, t120: f64, t33: f64, t58: f64, t118: f64, t169: f64) -> (f64, f64, f64) {
    let t174 = t129 * t173;
    let t177 = -t33 + t58 + 0.10427789137624512459e-2_f64 * t120 * t174;
    let t178 = t169 * t118;
    (t174, t177, t178)
}
