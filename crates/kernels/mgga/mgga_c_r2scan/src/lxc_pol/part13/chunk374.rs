//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 374/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk374<F: Float>(t1399: F, t1402: F, t1404: F, t1407: F, t1393: F, t1396: F, t401: F, t384: F) -> (F, F, F, F, F, F, F) {
    let t1473 = F::cast_from(0.39862222222222222223e0_f64) * t1399;
    let t1474 = F::cast_from(0.68258333333333333333e-1_f64) * t1402;
    let t1475 = F::cast_from(0.13651666666666666667e0_f64) * t1404;
    let t1476 = F::cast_from(0.13692777777777777778e0_f64) * t1407;
    let t1477 = -F::cast_from(0.42198333333333333333e0_f64) * t1393 + F::cast_from(0.84396666666666666666e0_f64) * t1396 + t1473 + t1474 + t1475 + t1476;
    let t1478 = t1477 * t401;
    let t1479 = t384 * t1478;
    let t1480 = F::new(1.0) * t1479;
    (t1473, t1474, t1475, t1476, t1477, t1478, t1480)
}
