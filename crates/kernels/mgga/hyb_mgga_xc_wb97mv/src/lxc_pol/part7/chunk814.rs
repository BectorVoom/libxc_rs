//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 814/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk814<F: Float>(t1128: F, t4619: F, t1143: F, t4533: F, t4541: F, t509: F, t4554: F, t513: F, t1117: F, t1127: F, t1132: F, t1136: F, t1148: F, t1158: F, t1161: F, t1164: F, t1520: F, t1523: F, t2860: F, t2900: F, t2915: F, t4588: F, t4591: F, t4594: F, t4597: F, t4600: F, t4603: F, t4608: F, t4610: F, t4613: F, t4616: F, t505: F, t507: F, t511: F, t529: F) -> (F, F, F, F, F) {
    let t4620 = t1128 * t4619;
    let t4631 = t1143 * t4533;
    let t4636 = t509 * t4541;
    let t4639 = t513 * t4554;
    let t4641 = 6.0 * t511 * t4588 + 30.0 * t2860 * t4591 + 6.0 * t1148 * t4594 - 6.0 * t529 * t4597 + 2.0 * t1117 * t4600 - 2.0 * t511 * t4603 - 2.0 * t1520 * t1523 + t4608 * t507 - 0.96e-1 * t1158 * t4610 + 0.96e-1 * t1161 * t4613 + 0.384e-6 * t1164 * t4616 - 0.24e-1 * t2900 * t4620 - 0.32e-1 * t1127 * t4610 + 0.32e-1 * t1132 * t4613 + 0.384e-6 * t1136 * t4616 - 0.216e0 * t2915 * t4620 - 0.88e-4 * t1136 * t4631 - 0.88e-4 * t1164 * t4631 + 2.0 * t4636 * t513 - t505 * t4639;
    (t4620, t4631, t4636, t4639, t4641)
}
