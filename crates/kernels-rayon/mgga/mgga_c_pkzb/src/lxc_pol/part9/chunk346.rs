//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 346/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk346(t1201: f64, t378: f64, t1169: f64, t1175: f64, t1178: f64, t1182: f64, t884: f64, t887: f64) -> (f64, f64) {
    let t1202 = t1201 * t378;
    let t1208 = 0.258925e1_f64 * t1175 - t884 + 0.905775e0_f64 * t1169 + 0.16504875e0_f64 * t1178 - t887 + 0.248355e0_f64 * t1182;
    (t1202, t1208)
}
