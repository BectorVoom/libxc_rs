//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 483/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk483<F: Float>(t2058: F, t6: F, t133: F, t542: F, t39: F, t550: F) -> (F, F, F, F) {
    let t8832 = t2058 * t6;
    let t8833 = t133 * t8832;
    let t8838 = t542 * t8832;
    let t8851 = t550 * t39;
    let t8852 = t133 * t8851;
    (t8833, t8838, t8851, t8852)
}
