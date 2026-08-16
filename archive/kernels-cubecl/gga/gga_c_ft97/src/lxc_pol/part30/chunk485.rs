//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 485/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk485<F: Float>(t295: F, t312: F, t7662: F, t1501: F, t6353: F, t296: F) -> (F, F, F, F) {
    let t7664 = t295 * t7662 * t312;
    let t7668 = t6353 * t1501;
    let t7669 = t296 * t7668;
    let t7672 = t1501 * t1501;
    (t7664, t7668, t7669, t7672)
}
