//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 684/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk684(t13261: f64, t2343: f64, t2268: f64, t3565: f64, t6470: f64, t882: f64, t11264: f64, t2492: f64, t11172: f64, t6485: f64, t883: f64, t11259: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13262 = t2343 * t13261;
    let t13264 = 0.56910013271352299198e-1_f64 * t2268 * t13262;
    let t13265 = t3565 * t6470;
    let t13266 = t882 * t13265;
    let t13267 = 0.35568758294595186999e-2_f64 * t13266;
    let t13268 = t11264 * t2492;
    let t13270 = 0.34146007962811379518e0_f64 * t2268 * t13268;
    let t13273 = t6485 * t883 * t11172;
    let t13274 = t882 * t13273;
    let t13275 = 0.23712505529730124666e-2_f64 * t13274;
    let t13276 = t11259 * t874;
    (t13262, t13264, t13265, t13267, t13268, t13270, t13273, t13275, t13276)
}
