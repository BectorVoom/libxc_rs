//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 905/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk905<F: Float>(t33289: F, t9800: F, t9806: F, t43107: F, t5241: F, t5640: F, t590: F, t11068: F, t2679: F, t9796: F, t33308: F, t9805: F) -> (F, F, F, F) {
    let t43389 = t9800 * t33289 * t9806;
    let t43398 = F::new(0.15337170381568299871e1) * t5640 * t5241 * t43107 * t590;
    let t43400 = t9796 * t11068 * t2679;
    let t43403 = t9805 * t33308 * t9806;
    (t43389, t43398, t43400, t43403)
}
