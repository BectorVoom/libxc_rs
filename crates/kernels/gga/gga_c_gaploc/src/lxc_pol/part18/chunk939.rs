//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 939/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk939<F: Float>(t11057: F, t9796: F, t10627: F, t5241: F, t590: F, t5640: F, t1890: F, t1966: F, t739: F, t1991: F, t1628: F, t3495: F, t1589: F, t3451: F, t3464: F, t769: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11058 = t9796 * t11057;
    let t11059 = 0.38342925953920749676e0 * t11058;
    let t11061 = t5241 * t10627 * t590;
    let t11063 = 0.15337170381568299871e1 * t5640 * t11061;
    let t11065 = t1890 * t10627 * t590;
    let t11067 = 0.25561950635947166451e1 * t1966 * t11065;
    let t11068 = t739 * t10627;
    let t11069 = t11068 * t590;
    let t11071 = 0.1022478025437886658e1 * t1991 * t11069;
    let t11072 = t1628 * t3495;
    let t11075 = t1589 * t3451;
    let t11080 = t769 * t3464;
    (t11059, t11061, t11063, t11065, t11067, t11069, t11071, t11072, t11075, t11080)
}
