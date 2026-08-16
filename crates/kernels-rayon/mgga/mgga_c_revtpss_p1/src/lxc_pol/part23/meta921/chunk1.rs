//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2972/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2972(t78303: f64, t78305: f64, t78307: f64, t78309: f64, t78311: f64, t78313: f64, t78315: f64, t78319: f64, t78322: f64, t78325: f64, t78682: f64, t78683: f64, t78699: f64, t78718: f64) -> f64 {
    let t78721 = t78682 + t78683 + t78699 - t78303 + t78305 - t78307 + t78309 - t78311 + t78313 + t78315 + t78319 - t78322 - t78325 + t78718;
    t78721
}
