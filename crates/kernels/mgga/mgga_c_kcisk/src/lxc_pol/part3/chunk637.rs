//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 637/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk637<F: Float>(t4304: F, t79: F, t4208: F, t469: F, t41: F, t470: F, t3784: F, t4229: F, t499: F, t260: F, t338: F, t67: F) -> (F, F, F, F, F, F) {
    let t6322 = t79 * t4304;
    let t6331 = t4208 * t469;
    let t6332 = t41 * t470;
    let t6368 = t3784 * t4229;
    let t6369 = t79 * t499;
    let t6442 = t260 * t67 * t338;
    (t6322, t6331, t6332, t6368, t6369, t6442)
}
