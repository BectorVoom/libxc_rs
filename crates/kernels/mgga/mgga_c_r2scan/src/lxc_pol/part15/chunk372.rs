//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 372/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk372<F: Float>(t1518: F, t2: F, t424: F, t464: F, t1381: F, t1497: F, t453: F, t234: F) -> (F, F, F, F, F, F) {
    let t1519 = 0.19751673498613801407e-1 * t1518;
    let t1520 = t424 * t2;
    let t1521 = t1520 * t464;
    let t1522 = 0.36622894612013090108e-3 * t1521;
    let t1524 = t1497 * t1381 * t453;
    let t1525 = t234 * t1524;
    let t1526 = 0.11696447245269292414e1 * t1525;
    (t1519, t1520, t1521, t1522, t1524, t1526)
}
