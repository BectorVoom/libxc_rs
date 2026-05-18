//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 787/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk787<F: Float>(t30334: F, t544: F, t9287: F, t9291: F, t9562: F, t20556: F, t587: F, t9438: F, t20967: F, t12454: F, t4391: F, t549: F) -> (F, F, F, F, F, F) {
    let t40251 = t544 * t30334;
    let t40252 = t40251 * t9287;
    let t40258 = t9291 * t9562;
    let t40261 = t587 * t9438 * t20556;
    let t40277 = t9291 * t20967;
    let t40280 = t4391 * t549 * t12454;
    (t40251, t40252, t40258, t40261, t40277, t40280)
}
