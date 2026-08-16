//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 581/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk581(t10254: f64, t2787: f64, t6519: f64, t2343: f64, t1063: f64, t2312: f64, t3344: f64, t2321: f64, t2822: f64, t882: f64, t2765: f64, t6750: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10255 = 0.11856252764865062333e-2_f64 * t10254;
    let t10256 = t2787 * t6519;
    let t10257 = t2343 * t10256;
    let t10259 = 0.56910013271352299198e-1_f64 * t1063 * t10257;
    let t10260 = t2312 * t3344;
    let t10261 = 0.11856252764865062333e-2_f64 * t10260;
    let t10262 = t2822 * t2321;
    let t10263 = t882 * t10262;
    let t10264 = 0.11856252764865062333e-2_f64 * t10263;
    let t10265 = t2765 * t6750;
    (t10255, t10256, t10259, t10261, t10264, t10265)
}
