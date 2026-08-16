//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 293/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk293<F: Float>(t1494: F, t209: F, t469: F, t6: F, t1193: F, t1466: F, t476: F, t605: F, t221: F, t589: F, t1228: F, t612: F) -> (F, F, F, F, F, F, F, F) {
    let t1497 = t469 * t6 * t1494 * t209;
    let t1500 = t1193 * t1466;
    let t1501 = t605 * t476;
    let t1502 = t1501 * t209;
    let t1503 = t221 * t1502;
    let t1508 = t589 * t476;
    let t1509 = t1508 * t209;
    let t1510 = t221 * t1509;
    let t1513 = t1228 * t612;
    (t1497, t1500, t1501, t1502, t1503, t1508, t1510, t1513)
}
