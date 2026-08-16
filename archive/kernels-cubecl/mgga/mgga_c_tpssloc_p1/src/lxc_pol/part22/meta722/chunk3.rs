//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2359/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2359<F: Float>(t262: F, t5527: F, t193: F, t202: F, t39585: F, t39590: F, t4119: F, t67322: F, t67457: F, t67458: F, t67461: F, t67464: F, t67466: F, t67472: F, t67475: F, t68305: F, t68333: F, t68365: F, t870: F) -> F {
    let t68371 = t5527 * t262;
    let t68375 = t193 * t202 * (t67322 + t68305 + t68333 + t68365) * t870 + t67457 + t67458 + t67461 + t67464 + t67466 + F::cast_from(18.0_f64) * t193 * t68371 * t4119 - t39585 + t39590 + t67472 + t67475;
    t68375
}
