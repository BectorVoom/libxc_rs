//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 549/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk549<F: Float>(t66: F, t8051: F, t391: F, t625: F, t68: F, t72: F, t2247: F, t47: F, t1557: F, t422: F, t1736: F, t7800: F, t424: F, t626: F, t419: F, t1570: F, t23: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8052 = t66 * t8051;
    let t8074 = t68 * t391 * t625 * t72;
    let t8076 = t47 * t2247;
    let t8078 = t68 * t8076 * t72;
    let t8079 = 0.70937342644032921812e-2 * t8078;
    let t8088 = t422 * t1557;
    let t8101 = t1736 * t7800;
    let t8109 = t626 * t424;
    let t8110 = t419 * t8109;
    let t8119 = 1.0 / t23 / t1570;
    (t8052, t8074, t8076, t8078, t8079, t8088, t8101, t8110, t8119)
}
