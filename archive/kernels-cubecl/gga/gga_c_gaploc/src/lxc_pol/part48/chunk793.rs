//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 793/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk793<F: Float>(t10215: F, t1429: F, t2365: F, t2366: F, t20535: F, t34688: F, t9537: F, t20671: F, t31037: F, t35101: F, t10205: F, t871: F) -> (F, F, F, F) {
    let t42026 = t1429 * t2365 * t2366 * t10215;
    let t42066 = t20535 * t34688 * t9537;
    let t42071 = t31037 * t20671 * t35101;
    let t42114 = t10205 * t871;
    (t42026, t42066, t42071, t42114)
}
