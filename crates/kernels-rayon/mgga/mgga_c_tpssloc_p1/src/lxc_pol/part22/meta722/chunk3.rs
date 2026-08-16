//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2359/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2359(t262: f64, t5527: f64, t193: f64, t202: f64, t39585: f64, t39590: f64, t4119: f64, t67322: f64, t67457: f64, t67458: f64, t67461: f64, t67464: f64, t67466: f64, t67472: f64, t67475: f64, t68305: f64, t68333: f64, t68365: f64, t870: f64) -> f64 {
    let t68371 = t5527 * t262;
    let t68375 = t193 * t202 * (t67322 + t68305 + t68333 + t68365) * t870 + t67457 + t67458 + t67461 + t67464 + t67466 + 18.0_f64 * t193 * t68371 * t4119 - t39585 + t39590 + t67472 + t67475;
    t68375
}
