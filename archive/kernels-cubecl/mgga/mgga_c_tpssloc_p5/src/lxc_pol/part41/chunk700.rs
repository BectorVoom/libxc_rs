//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 700/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk700<F: Float>(t1044: F, t248: F, t4347: F, t1009: F, t1603: F, t1011: F, t1019: F) -> (F, F, F, F) {
    let t4636 = t248 * t1044 * t4347;
    let t4639 = t1603 * t1009;
    let t4640 = t4639 * t1011;
    let t4641 = t4640 * t1019;
    (t4636, t4639, t4640, t4641)
}
