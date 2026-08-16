//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 666/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk666(t11053: f64, t2679: f64, t9805: f64, t1029: f64, t9796: f64, t10627: f64, t5241: f64, t590: f64, t5640: f64, t1890: f64, t1966: f64, t739: f64) -> (f64, f64, f64, f64, f64) {
    let t11054 = t11053 * t2679;
    let t11055 = t9805 * t11054;
    let t11056 = 0.57514388930881124514e0_f64 * t11055;
    let t11057 = t1029 * t2679;
    let t11058 = t9796 * t11057;
    let t11059 = 0.38342925953920749676e0_f64 * t11058;
    let t11061 = t5241 * t10627 * t590;
    let t11063 = 0.15337170381568299871e1_f64 * t5640 * t11061;
    let t11065 = t1890 * t10627 * t590;
    let t11067 = 0.25561950635947166451e1_f64 * t1966 * t11065;
    let t11068 = t739 * t10627;
    (t11056, t11059, t11063, t11067, t11068)
}
