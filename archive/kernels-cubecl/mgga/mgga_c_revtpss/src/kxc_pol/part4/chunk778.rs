//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 778/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk778<F: Float>(t1398: F, t675: F, t268: F, t543: F, t4101: F, t1419: F, t72: F, t1432: F, t686: F, t1433: F, t2470: F, t3999: F, t555: F) -> (F, F, F, F, F, F) {
    let t4102 = t675 * t1398;
    let t4104 = t268 * t4102 * t543;
    let t4105 = t4101 * t4104;
    let t4107 = t1419 * t72;
    let t4109 = t1432 * t4107 * t686;
    let t4113 = F::cast_from(0.13009920719177044025e-1_f64) * t1432 * t1433 * t2470;
    let t4114 = t3999 * t555;
    (t4104, t4105, t4107, t4109, t4113, t4114)
}
