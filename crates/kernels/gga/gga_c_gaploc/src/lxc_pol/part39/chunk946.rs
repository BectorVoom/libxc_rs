//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 946/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk946<F: Float>(t12987: F, t7014: F, t2365: F, t31558: F, t7025: F, t1645: F, t2859: F, t9152: F, t3149: F, t8063: F, t2877: F, t9487: F) -> (F, F, F, F, F) {
    let t42256 = t7014 * t12987;
    let t42257 = F::cast_from(0.15976219147466979032e-1_f64) * t42256;
    let t42259 = t7025 * t2365 * t31558;
    let t42263 = F::cast_from(0.10725146985555128001e1_f64) * t2859 * t1645 * t9152;
    let t42265 = F::cast_from(0.23833659967900284446e0_f64) * t3149 * t8063;
    let t42267 = F::cast_from(0.35750489951850426669e0_f64) * t9487 * t2877;
    (t42257, t42259, t42263, t42265, t42267)
}
