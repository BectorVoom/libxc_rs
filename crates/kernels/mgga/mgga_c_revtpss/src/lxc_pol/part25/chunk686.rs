//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 686/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk686<F: Float>(t4086: F, t555: F, t786: F, t1398: F, t675: F, t268: F, t543: F, t1419: F, t72: F, t1432: F, t686: F, t1433: F, t2470: F) -> (F, F, F, F, F, F, F, F) {
    let t4100 = t4086 * t555;
    let t4101 = t786 * t4100;
    let t4102 = t675 * t1398;
    let t4104 = t268 * t4102 * t543;
    let t4105 = t4101 * t4104;
    let t4107 = t1419 * t72;
    let t4109 = t1432 * t4107 * t686;
    let t4113 = F::cast_from(0.13009920719177044025e-1_f64) * t1432 * t1433 * t2470;
    (t4100, t4101, t4102, t4104, t4105, t4107, t4109, t4113)
}
