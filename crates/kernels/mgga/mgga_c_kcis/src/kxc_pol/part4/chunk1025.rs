//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1025/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1025<F: Float>(t14435: F, t14482: F, t14531: F, t14561: F, t1009: F, t1014: F, t4925: F, t4768: F, t978: F, t2846: F, t4999: F, t2842: F, t2861: F, t4986: F, t1773: F, t3316: F) -> (F, F, F, F, F, F, F, F) {
    let t14563 = t14435 + t14482 + t14531 + t14561;
    let t14564 = t14563 * t1009;
    let t14567 = t1014 * t4925;
    let t14568 = 0.33163888888888888888e-2 * t14567;
    let t14570 = t4768 * t978;
    let t14573 = t4999 * t2846;
    let t14574 = t2842 * t14573;
    let t14576 = t2861 * t4986;
    let t14577 = 0.22109259259259259258e-2 * t14576;
    let t14578 = t1773 * t3316;
    (t14564, t14567, t14568, t14570, t14574, t14576, t14577, t14578)
}
