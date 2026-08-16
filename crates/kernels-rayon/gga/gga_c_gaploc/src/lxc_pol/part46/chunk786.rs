//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 786/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk786(t12507: f64, t4379: f64, t1429: f64, t2365: f64, t2366: f64, t9127: f64, t12538: f64, t1407: f64, t2464: f64, t2465: f64, t587: f64, t9316: f64) -> (f64, f64, f64, f64) {
    let t40239 = t4379 * t12507;
    let t40243 = t1429 * t2365 * t2366 * t9127;
    let t40245 = t1407 * t12538;
    let t40249 = t587 * t2464 * t2465 * t9316;
    (t40239, t40243, t40245, t40249)
}
