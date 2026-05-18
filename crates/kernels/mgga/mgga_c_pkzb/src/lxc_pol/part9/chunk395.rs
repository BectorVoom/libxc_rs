//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 395/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk395<F: Float>(t501: F, t546: F, t496: F, t513: F, t1501: F, t1510: F, t1513: F, t1520: F, t1530: F, t1534: F, t1535: F, t1536: F, t1544: F, t1547: F, t1550: F, t1553: F, t568: F) -> (F, F, F, F, F) {
    let t1555 = F::new(8.0) * t501 * t546;
    let t1556 = t496 * t513;
    let t1557 = F::new(8.0) * t1556;
    let t1559 = F::new(8.0) * t496 * t546;
    let t1560 = F::new(6.0) * t1535 * t1536 * t568 - t1501 - t1510 - t1513 - t1520 + t1530 + t1534 + t1544 + t1547 - t1550 - t1553 - t1555 + t1557 + t1559;
    (t1555, t1556, t1557, t1559, t1560)
}
