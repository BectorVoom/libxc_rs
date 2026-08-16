//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2177/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2177<F: Float>(t1410: F, t9231: F, t2240: F, t3961: F, t3967: F, t12571: F, t608: F, t33: F, t46099: F, t2244: F, t3953: F, t9239: F) -> (F, F, F, F, F, F, F) {
    let t90098 = t9231 * t1410;
    let t90101 = t2240 * t3961;
    let t90104 = t2240 * t3967;
    let t90114 = t12571 * t608;
    let t90121 = t46099 * t33;
    let t90132 = t3953 * t2244;
    let t90137 = t9239 * t1410;
    (t90098, t90101, t90104, t90114, t90121, t90132, t90137)
}
