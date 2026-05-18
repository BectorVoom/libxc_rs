//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 576/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk576<F: Float>(t444: F, t5811: F, t2001: F, t129: F, t1691: F, t14: F, t549: F, t72: F, t5828: F, t542: F, t550: F, t133: F) -> (F, F, F, F, F, F, F, F) {
    let t23714 = t5811 * t444;
    let t23715 = t2001 * t23714;
    let t23721 = t129 * t1691;
    let t23724 = t549 * t14;
    let t23725 = t23724 * t72;
    let t23732 = t2001 * t5828;
    let t23742 = t542 * t550;
    let t23745 = t133 * t550;
    (t23714, t23715, t23721, t23724, t23725, t23732, t23742, t23745)
}
