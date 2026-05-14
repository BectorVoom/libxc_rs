//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 602/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk602<F: Float>(t3426: F, t83: F, t1501: F, t1510: F, t1555: F, t1627: F, t3382: F, t3421: F, t3422: F, t3423: F, t3424: F, t3425: F, t1520: F, t1530: F, t1534: F, t1544: F, t1547: F, t1550: F, t1553: F, t1559: F, t1596: F, t1604: F, t1641: F, t1669: F) -> (F, F, F) {
    let t3427 = t83 * t3426;
    let t3428 = -t3421 + t3422 - t3423 - t3424 - t3425 + t3427 + t3382 + t1627 - t1501 - t1510 - t1555;
    let t3429 = -t1559 - t1520 + t1530 + t1604 + t1669 + t1641 - t1596 + t1544 + t1547 + t1550 - t1553 + t1534;
    (t3427, t3428, t3429)
}
