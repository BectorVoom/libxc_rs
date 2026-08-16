//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 42/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk42(t24: f64, t91: f64, t86: f64, t89: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let cbrt2 = (M_CBRT2 as f64);
    let t90 = t24 <= zeta_threshold;
    let t92 = t91 * t24;
    let t93 = piecewise3(t90, t86, t92);
    let t94 = t89 + t93 - 2.0_f64;
    let t95 = cbrt2;
    (t92, t94, t95)
}
