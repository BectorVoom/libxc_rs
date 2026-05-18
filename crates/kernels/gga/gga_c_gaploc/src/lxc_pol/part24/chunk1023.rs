//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1023/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1023<F: Float>(t11053: F, t2679: F, t9805: F, t1029: F, t9796: F, t10627: F, t5241: F, t590: F, t5640: F, t1890: F, t1966: F, t739: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11054 = t11053 * t2679;
    let t11055 = t9805 * t11054;
    let t11056 = F::new(0.57514388930881124514e0) * t11055;
    let t11057 = t1029 * t2679;
    let t11058 = t9796 * t11057;
    let t11059 = F::new(0.38342925953920749676e0) * t11058;
    let t11061 = t5241 * t10627 * t590;
    let t11063 = F::new(0.15337170381568299871e1) * t5640 * t11061;
    let t11065 = t1890 * t10627 * t590;
    let t11067 = F::new(0.25561950635947166451e1) * t1966 * t11065;
    let t11068 = t739 * t10627;
    (t11054, t11056, t11057, t11059, t11061, t11063, t11065, t11067, t11068)
}
