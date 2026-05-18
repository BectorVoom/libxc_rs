//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 624/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk624<F: Float>(t1547: F, t1550: F, t1553: F, t1555: F, t1559: F, t1596: F, t1604: F, t1627: F, t1641: F, t1669: F, t3421: F, t3423: F, t3424: F, t3425: F) -> F {
    let t3506 = t1547 + t1550 - t3421 - t1553 - t1555 - t1559 + t1627 - t3423 - t3424 + t1604 + t1641 - t1596 + t1669 - t3425;
    t3506
}
