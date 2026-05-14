//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 613/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk613<F: Float>(t448: F, t4752: F, t1306: F, t1645: F, t1616: F, t596: F, t1328: F, t165: F, t1559: F, t19: F) -> (F, F, F, F, F) {
    let t4753 = t4752 * t448;
    let t4762 = t1645 * t1306;
    let t4771 = t1616 * t596;
    let t4774 = t165 * t1328;
    let t4779 = t1559 * t19;
    (t4753, t4762, t4771, t4774, t4779)
}
