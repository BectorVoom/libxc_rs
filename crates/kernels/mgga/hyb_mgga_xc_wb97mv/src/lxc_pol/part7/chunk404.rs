//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 404/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk404<F: Float>(t1153: F, t1522: F, t1117: F, t1127: F, t1132: F, t1136: F, t1148: F, t1158: F, t1161: F, t1164: F, t1520: F, t1523: F, t1525: F, t1528: F, t1531: F, t1534: F, t1538: F, t1541: F, t1544: F, t505: F, t507: F, t511: F, t529: F) -> (F, F) {
    let t1547 = t1153 * t1522;
    let t1558 = t1520 * t507 - t505 * t1523 + 2.0 * t1117 * t1525 - 2.0 * t511 * t1528 + 0.6e-2 * t1127 * t1531 - 0.6e-2 * t1132 * t1534 - 0.8e-2 * t1136 * t1538 + 0.24e-4 * t1136 * t1541 + 6.0 * t1148 * t1544 - 6.0 * t529 * t1547 + 0.18e-1 * t1158 * t1531 - 0.18e-1 * t1161 * t1534 - 0.8e-2 * t1164 * t1538 + 0.24e-4 * t1164 * t1541;
    (t1547, t1558)
}
