//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 567/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk567<F: Float>(t4753: F, t600: F, t1670: F, t45: F, t1675: F, t596: F, t1683: F) -> (F, F, F, F) {
    let t4754 = t4753 * t600;
    let t4757 = t45 * t1670;
    let t4760 = t1675 * t596;
    let t4761 = F::new(1.0) / t4760;
    let t4762 = t1683 * t1683;
    (t4754, t4757, t4761, t4762)
}
