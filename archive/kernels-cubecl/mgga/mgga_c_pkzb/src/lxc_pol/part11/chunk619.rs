//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 619/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk619<F: Float>(t1520: F, t1530: F, t1534: F, t1544: F, t1547: F, t1550: F, t1553: F, t1559: F, t1596: F, t1604: F, t1641: F, t1669: F) -> F {
    let t3429 = -t1559 - t1520 + t1530 + t1604 + t1669 + t1641 - t1596 + t1544 + t1547 + t1550 - t1553 + t1534;
    t3429
}
