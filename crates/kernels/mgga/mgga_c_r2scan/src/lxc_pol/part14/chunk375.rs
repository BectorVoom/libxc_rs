//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 375/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk375<F: Float>(t1399: F, t1402: F, t1404: F, t1407: F, t1393: F, t1396: F, t401: F, t384: F) -> (F, F, F, F, F, F, F) {
    let t1473 = F::new(0.39862222222222222223e0) * t1399;
    let t1474 = F::new(0.68258333333333333333e-1) * t1402;
    let t1475 = F::new(0.13651666666666666667e0) * t1404;
    let t1476 = F::new(0.13692777777777777778e0) * t1407;
    let t1477 = -F::new(0.42198333333333333333e0) * t1393 + F::new(0.84396666666666666666e0) * t1396 + t1473 + t1474 + t1475 + t1476;
    let t1478 = t1477 * t401;
    let t1479 = t384 * t1478;
    let t1480 = F::new(1.0) * t1479;
    (t1473, t1474, t1475, t1476, t1477, t1478, t1480)
}
