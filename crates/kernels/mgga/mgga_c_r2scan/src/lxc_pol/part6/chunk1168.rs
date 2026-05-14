//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1168/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1168<F: Float>(t1691: F, t1813: F, t1835: F, t61: F, t424: F, t5435: F, t704: F, t5385: F, t745: F, t1696: F, t1745: F, t1732: F, t5951: F, t5961: F, t5967: F, t1762: F, t1763: F, t5798: F) -> (F, F, F, F, F, F, F, F) {
    let t21311 = t1813 * t1691;
    let t21313 = 0.69350015718254262349e2 * t61 * t1835 * t21311;
    let t21315 = t424 * t704 * t5435;
    let t21320 = t5385 * t745;
    let t21322 = t1696 * t1745;
    let t21324 = t5951 * t1732;
    let t21326 = t5967 * t5961;
    let t21330 = 0.1301229756036208781e0 * t1762 * t1763 * t5798;
    (t21311, t21313, t21315, t21320, t21322, t21324, t21326, t21330)
}
