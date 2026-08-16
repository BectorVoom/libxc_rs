//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2489/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2489(t2979: f64, t43248: f64, t50259: f64, t50263: f64, t62657: f64, t62660: f64, t62663: f64, t62666: f64, t62682: f64, t62687: f64, t68462: f64, t68481: f64, t973: f64) -> f64 {
    let t70837 = -t62657 / 36.0_f64 + t62660 / 108.0_f64 - t62663 / 144.0_f64 + t62666 / 216.0_f64 + t973 * t2979 * t68481 / 6.0_f64 - t973 * t2979 * t68462 / 12.0_f64 + t50259 - t50263 + t62682 / 1152.0_f64 - t62687 / 576.0_f64 - t43248 / 1944.0_f64;
    t70837
}
