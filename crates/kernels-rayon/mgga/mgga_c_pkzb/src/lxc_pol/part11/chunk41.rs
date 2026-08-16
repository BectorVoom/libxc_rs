//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 41/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk41(t12: f64, t87: f64, t86: f64, t24: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t84 = t12 <= zeta_threshold;
    let t88 = t87 * t12;
    let t89 = piecewise3(t84, t86, t88);
    let t90 = t24 <= zeta_threshold;
    let t91 = pow_1_3(t24);
    (t88, t89, t91)
}
