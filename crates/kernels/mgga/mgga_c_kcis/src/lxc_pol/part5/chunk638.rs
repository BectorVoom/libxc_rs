//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 638/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk638<F: Float>(t2970: F, t4567: F, t26: F, t4581: F, t945: F, t22: F, t2470: F) -> (F, F, F, F, F) {
    let t4708 = t2970 * t4567;
    let t4709 = t26 * t4708;
    let t4711 = t945 * t4581;
    let t4712 = t26 * t4711;
    let t4714 = t22 * t2470;
    (t4708, t4709, t4711, t4712, t4714)
}
