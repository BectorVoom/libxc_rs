//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 490/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk490<F: Float>(t12: F, t20: F, t1392: F, t1395: F, t640: F, t1399: F, t1732: F, t1734: F) -> (F, F, F, F, F) {
    let t1736 = 1.0/pow_3_2(t12);
    let t1737 = t1736 * t20;
    let t1738 = t1737 * t1392;
    let t1740 = t640 * t1395;
    let t1743 = 0.17261666666666666666e1 * t1732 - 0.46031111111111111111e1 * t1734 - 0.73354999999999999999e-1 * t1738 + 0.14671e0 * t1740 + 0.11038e0 * t1399;
    (t1736, t1737, t1738, t1740, t1743)
}
