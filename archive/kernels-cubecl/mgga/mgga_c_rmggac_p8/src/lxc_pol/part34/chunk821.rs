//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 821/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk821<F: Float>(t70221: F, t8571: F, t68555: F, t15379: F, t68929: F, t34938: F, t656: F, t8963: F, t34944: F, t8937: F, t36471: F, t8947: F) -> (F, F, F, F, F, F) {
    let t74749 = t8571 * t70221;
    let t74751 = t8571 * t68555;
    let t74753 = t15379 * t68929;
    let t74756 = t34938 * t656 * t8963;
    let t74759 = t34944 * t656 * t8937;
    let t74762 = t36471 * t656 * t8947;
    (t74749, t74751, t74753, t74756, t74759, t74762)
}
