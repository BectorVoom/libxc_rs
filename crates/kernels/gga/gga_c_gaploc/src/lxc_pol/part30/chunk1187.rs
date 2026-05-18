//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1187/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1187<F: Float>(t31878: F, t10185: F, t29874: F, t10257: F, t3818: F, t20896: F, t2268: F, t7937: F, t2325: F, t25556: F, t882: F, t883: F) -> (F, F, F, F, F) {
    let t31879 = F::new(0.63233348079280332443e-2) * t31878;
    let t31880 = t29874 * t10185;
    let t31881 = F::new(0.47425011059460249332e-2) * t31880;
    let t31883 = F::new(0.15176003539027279786e0) * t3818 * t10257;
    let t31886 = F::new(0.34146007962811379518e0) * t2268 * t7937 * t20896;
    let t31889 = t882 * t2325 * t883 * t25556;
    (t31879, t31881, t31883, t31886, t31889)
}
