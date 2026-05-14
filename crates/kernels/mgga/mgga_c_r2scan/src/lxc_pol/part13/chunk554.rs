//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 554/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk554<F: Float>(t761: F, t955: F, t759: F, t2060: F, t897: F, t2062: F, t1421: F, t1459: F, t1463: F, t1470: F, t1480: F, t1488: F, t1511: F, t1513: F, t1526: F, t1529: F, t1533: F, t246: F, t2461: F, t2490: F, t2492: F, t2494: F, t2495: F) -> (F, F) {
    let t2820 = t955 * t761;
    let t2821 = t759 * t2820;
    let t2823 = t2060 * t897;
    let t2824 = t2823 * t2062;
    let t2828 = t1421 - t1511 + 0.285764e-1 * t2821 + t1459 - t1526 - 0.675260332e-1 * t2824 - t1513 + t2490 + t2492 + t1470 - t1480 - t1488 - 0.285764e-1 * t246 * t2461 - t2494 - t1529 + t1463 + t2495 - t1533;
    (t2823, t2828)
}
