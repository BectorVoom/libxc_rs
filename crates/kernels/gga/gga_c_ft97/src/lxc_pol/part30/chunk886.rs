//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 886/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk886<F: Float>(t296: F, t36066: F, t1212: F, t7672: F, t2843: F, t840: F, t36064: F, t1091: F, t34207: F, t2881: F, t24890: F, t7032: F) -> (F, F, F, F, F, F, F) {
    let t36130 = t296 * t36066;
    let t36133 = t7672 * t1212;
    let t36135 = t840 * t2843 * t36133;
    let t36138 = t296 * t36064;
    let t36141 = t34207 * t1091;
    let t36142 = t2881 * t36141;
    let t36145 = t24890 * t7032;
    (t36130, t36133, t36135, t36138, t36141, t36142, t36145)
}
