//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1004/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1004<F: Float>(t2092: F, t25894: F, t3677: F, t1471: F, t30294: F, t12: F) -> (F, F) {
    let t30565 = t25894 * t2092;
    let t30567 = F::cast_from(0.48245472966453314466e2_f64) * t3677 * t30565;
    let t30568 = t1471 * t30294;
    let t30569 = t12 * t30568;
    (t30567, t30569)
}
