//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1842/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1842<F: Float>(t16596: F, t23788: F, t1081: F, t1484: F, t28: F, t4119: F, t25365: F, t10143: F) -> (F, F, F, F, F) {
    let t25898 = t23788 * t16596;
    let t25901 = t1081 * t1484;
    let t25905 = t28 * t4119;
    let t25921 = t23788 * t25365;
    let t25927 = t10143 * t28;
    (t25898, t25901, t25905, t25921, t25927)
}
