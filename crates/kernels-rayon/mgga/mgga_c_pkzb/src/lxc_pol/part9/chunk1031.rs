//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1031/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1031(t2368: f64, t394: f64, t8429: f64, t406: f64, t3208: f64, t926: f64, t3206: f64, t2382: f64, t3265: f64, t2381: f64, t1249: f64, t6483: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8451 = t2368 * t394;
    let t8452 = t8429 * t8451;
    let t8453 = t406 * t8452;
    let t8456 = t926 * t3208;
    let t8458 = 0.28582678745379824648e-3_f64 * t3206 * t8456;
    let t8459 = t3265 * t2382;
    let t8460 = t2381 * t8459;
    let t8463 = t1249 * t6483;
    (t8451, t8452, t8453, t8458, t8459, t8460, t8463)
}
