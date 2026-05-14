//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 810/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk810<F: Float>(t1522: F, t3711: F, t2848: F, t4529: F, t1122: F, t1117: F, t1127: F, t1128: F, t1132: F, t1136: F, t1148: F, t1158: F, t1161: F, t1164: F, t2817: F, t2823: F, t2828: F, t2832: F, t2946: F, t2953: F, t2957: F, t3697: F, t4523: F, t4526: F, t4530: F, t4534: F, t4541: F, t4551: F, t4555: F, t4559: F, t4571: F, t505: F, t523: F, t529: F) -> (F, F, F, F) {
    let t4574 = t3711 * t1522;
    let t4581 = t2848 * t4529;
    let t4584 = t1122 * t4529;
    let t4587 = 0.96e-4 * t2817 * t4523 - 0.96e-4 * t2823 * t4526 + 0.126e0 * t2957 * t4530 + 0.29333333333333333333e-1 * t1164 * t4534 + 0.18e-1 * t2946 * t4530 + 0.29333333333333333333e-1 * t1136 * t4534 + 0.9e-1 * t2953 * t1128 * t4541 + 0.6e-2 * t1127 * t4551 - 0.6e-2 * t1132 * t4555 - 0.128e-3 * t1136 * t4559 + 0.18e-1 * t1158 * t4551 - 0.18e-1 * t1161 * t4555 - 0.128e-3 * t1164 * t4559 - 72.0 * t1148 * t3697 * t1522 + 0.6e-2 * t4571 * t523 - 8.0 * t1117 * t4574 + 0.288e-3 * t2828 * t4523 - 0.288e-3 * t2832 * t4526 + 42.0 * t529 * t4581 + 2.0 * t505 * t4584;
    (t4574, t4581, t4584, t4587)
}
