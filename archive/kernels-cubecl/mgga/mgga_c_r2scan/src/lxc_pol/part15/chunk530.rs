//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 530/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk530<F: Float>(t1515: F, t468: F, t963: F, t2483: F, t86: F, t1521: F, t1459: F, t1463: F, t1470: F, t1480: F, t1488: F, t1513: F, t1526: F, t1529: F, t1533: F) -> (F, F, F, F, F) {
    let t2490 = F::cast_from(4.0_f64) * t1515;
    let t2491 = t963 * t468;
    let t2492 = F::cast_from(0.5848223622634646207e0_f64) * t2491;
    let t2493 = t2483 * t86;
    let t2494 = F::cast_from(0.19751673498613801407e-1_f64) * t2493;
    let t2495 = F::cast_from(0.18311447306006545054e-3_f64) * t1521;
    let t2496 = -t1459 + t1526 + t1513 - t2490 - t2492 - t1470 + t1480 + t1488 + t2494 + t1529 - t1463 - t2495 + t1533;
    (t2490, t2492, t2494, t2495, t2496)
}
