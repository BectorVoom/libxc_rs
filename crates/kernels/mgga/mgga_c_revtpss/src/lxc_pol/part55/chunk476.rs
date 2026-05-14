//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 476/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk476<F: Float>(t136: F, t555: F, t2457: F, t3964: F, t4086: F, t786: F, t1398: F, t675: F, t268: F, t543: F, t1419: F, t72: F, t1432: F, t686: F, t1433: F, t2470: F) -> (F, F, F, F, F) {
    let t4096 = t555 * t136;
    let t4099 = 0.11565819519348392139e-2 * t3964 * t4096 * t2457;
    let t4100 = t4086 * t555;
    let t4101 = t786 * t4100;
    let t4102 = t675 * t1398;
    let t4104 = t268 * t4102 * t543;
    let t4105 = t4101 * t4104;
    let t4107 = t1419 * t72;
    let t4109 = t1432 * t4107 * t686;
    let t4113 = 0.13009920719177044025e-1 * t1432 * t1433 * t2470;
    (t4099, t4101, t4105, t4109, t4113)
}
