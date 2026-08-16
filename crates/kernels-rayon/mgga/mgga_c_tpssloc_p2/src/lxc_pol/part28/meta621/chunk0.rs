//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1942/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1942(t22779: f64, t26319: f64, t1358: f64, t26248: f64, t3862: f64, t7715: f64, t22705: f64, t22852: f64, t236: f64, t5286: f64, t550: f64, t26245: f64, t80791: f64) -> (f64, f64, f64, f64, f64) {
    let t91300 = t22779 * t26319;
    let t91303 = t26248 * t1358;
    let t91305 = t7715 * t3862;
    let t91310 = t22852 * t22705 * t236 * t5286 * t550;
    let t91312 = t80791 * t26245;
    (t91300, t91303, t91305, t91310, t91312)
}
