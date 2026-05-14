//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 393/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk393<F: Float>(t1435: F, t438: F, t1399: F, t1402: F, t1404: F, t1407: F, t1393: F, t1396: F) -> (F, F, F, F, F, F) {
    let t1436 = t1435 * t438;
    let t1441 = 0.68863333333333333333e0 * t1399;
    let t1442 = 0.14025833333333333333e0 * t1402;
    let t1443 = 0.28051666666666666667e0 * t1404;
    let t1444 = 0.17365833333333333333e0 * t1407;
    let t1445 = -0.78438333333333333333e0 * t1393 + 0.15687666666666666667e1 * t1396 + t1441 + t1442 + t1443 + t1444;
    (t1436, t1441, t1442, t1443, t1444, t1445)
}
