//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1086/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1086<F: Float>(t18574: F, t4579: F, t3269: F, t6334: F, t934: F, t3255: F, t6574: F, t6578: F, t1098: F, t6606: F, t6570: F, t6582: F) -> (F, F, F, F, F, F, F) {
    let t18575 = t4579 * t18574;
    let t18579 = t3269 * t6334 * t934;
    let t18582 = t3255 * t6574;
    let t18584 = t3255 * t6578;
    let t18586 = t1098 * t6606;
    let t18588 = t3255 * t6570;
    let t18590 = t3255 * t6582;
    (t18575, t18579, t18582, t18584, t18586, t18588, t18590)
}
