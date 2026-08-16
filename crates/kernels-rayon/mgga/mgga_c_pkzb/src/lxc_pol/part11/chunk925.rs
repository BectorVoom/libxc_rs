//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 925/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk925(t2382: f64, t3913: f64, t2381: f64, t3199: f64, t394: f64, t3186: f64, t406: f64, t3874: f64, t5728: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10066 = t3913 * t2382;
    let t10067 = t2381 * t10066;
    let t10070 = t394 * t3199;
    let t10071 = t3186 * t10070;
    let t10072 = t406 * t10071;
    let t10075 = t3874 * t5728;
    (t10066, t10067, t10070, t10071, t10072, t10075)
}
