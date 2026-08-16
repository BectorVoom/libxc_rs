//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1665/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1665(t12283: f64, t3809: f64, t3777: f64, t3789: f64, t12248: f64, t236: f64, t240: f64, t1336: f64, t12251: f64, t1343: f64, t820: f64, t12255: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12284 = t12283 * t3809;
    let t12286 = t3777 * t3789;
    let t12289 = t12248 * t236;
    let t12290 = t12289 * t240;
    let t12291 = t1336 * t12290;
    let t12293 = t1343 * t820 * t12251;
    let t12297 = t1343 * t820 * t12255;
    (t12284, t12286, t12289, t12290, t12291, t12293, t12297)
}
