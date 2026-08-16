//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1064/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1064(t12212: f64, t12233: f64, t12254: f64, t12275: f64, t12044: f64, t12045: f64, t12046: f64, t12054: f64, t12061: f64, t12152: f64, t12154: f64, t12155: f64, t12156: f64, t12158: f64, t12161: f64, t12162: f64, t12192: f64, t2464: f64, t3846: f64, t884: f64) -> (f64, f64) {
    let t12277 = t12212 + t12233 + t12254 + t12275;
    let t12279 = -t12277 * t884 - t2464 * t3846 - t12044 + t12045 + t12046 + t12054 + t12061 - t12152 - t12154 - t12155 - t12156 + t12158 + t12161 - t12162 + t12192;
    (t12277, t12279)
}
