//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 621/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk621(t1974: f64, t5392: f64, t1964: f64, t755: f64, t763: f64, t5374: f64, t1670: f64, t1676: f64, t4761: f64, t591: f64, t1685: f64, t4762: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5393 = t5392 * t1974;
    let t5396 = t1964 * t1964;
    let t5397 = 1.0_f64 / t5396;
    let t5398 = t755 * t5397;
    let t5399 = t763 * t763;
    let t5400 = 1.0_f64 / t5399;
    let t5401 = t5374 * t5400;
    let t5405 = t1670 * t1676;
    let t5408 = t591 * t4761;
    let t5409 = t4762 * t1685;
    (t5393, t5396, t5397, t5398, t5399, t5400, t5401, t5405, t5408, t5409)
}
