//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1274/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1274<F: Float>(t10153: F, t3727: F, t6182: F, t3243: F, t6188: F, t10245: F, t2531: F, t918: F, t11512: F, t2208: F, t22672: F, t2580: F) -> (F, F, F, F) {
    let t35986 = t10153 * t3727 * t6182;
    let t35989 = t3243 * t3727 * t6188;
    let t35992 = t10245 * t918 * t2531;
    let t35996 = t22672 * t2208 * t11512 * t2580;
    (t35986, t35989, t35992, t35996)
}
