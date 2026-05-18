//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 940/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk940<F: Float>(t40658: F, t7717: F, t38471: F, t7473: F, t7478: F, t35637: F, t8417: F, t1971: F, t236: F, t5620: F, t7365: F, t5624: F) -> (F, F, F, F, F) {
    let t40659 = t7717 * t40658;
    let t40661 = t38471 * t7473;
    let t40662 = t40661 * t7478;
    let t40664 = t35637 * t8417;
    let t40668 = t7365 * t1971 * t236 * t5620;
    let t40672 = t7365 * t1971 * t236 * t5624;
    (t40659, t40662, t40664, t40668, t40672)
}
