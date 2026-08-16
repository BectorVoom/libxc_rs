//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1089/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1089(t1084: f64, t291: f64, t33521: f64, t33527: f64, t4052: f64, t3095: f64, t6182: f64, t9438: f64, t33487: f64, t33492: f64, t33495: f64, t33501: f64, t33505: f64, t33507: f64, t33510: f64, t33513: f64, t33518: f64) -> (f64, f64) {
    let t33528 = t1084 * t4052 * t33521 * t291 * t33527;
    let t33530 = t3095 * t291;
    let t33532 = t9438 * t33530 * t6182;
    let t33534 = -0.687148483626368822e-6_f64 * t33487 - 0.12290803273518880209e-7_f64 * t33492 + 0.6670285450542344196e-8_f64 * t33495 - 0.13097074855481695406e-9_f64 * t33501 + 0.12290803273518880209e-8_f64 * t33505 + 0.33816362383187442026e-5_f64 * t33507 - 0.31675337336021900772e-5_f64 * t33510 - 0.24760339692676868218e-5_f64 * t33513 + 0.4834058140556728127e-8_f64 * t33518 + 0.14099336243290457037e-8_f64 * t33528 - 0.28960308421505737848e-5_f64 * t33532;
    (t33530, t33534)
}
