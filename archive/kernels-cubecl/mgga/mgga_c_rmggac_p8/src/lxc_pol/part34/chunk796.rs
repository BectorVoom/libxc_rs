//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 796/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk796<F: Float>(t14125: F, t68448: F, t73722: F, t3077: F, t38973: F, t68386: F, t7248: F, t8667: F, t8830: F, t9188: F, t3352: F, t8835: F) -> (F, F, F, F, F) {
    let t74319 = t68448 * t14125 * t73722;
    let t74321 = t38973 * t3077;
    let t74324 = t68386 * t7248 * t8667;
    let t74327 = t68386 * t9188 * t8830;
    let t74330 = t68386 * t3352 * t8835;
    (t74319, t74321, t74324, t74327, t74330)
}
