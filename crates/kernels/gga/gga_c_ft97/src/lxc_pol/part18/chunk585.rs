//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 585/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk585<F: Float>(t66: F, t8051: F, t391: F, t625: F, t68: F, t72: F, t2247: F, t47: F, t1675: F, t172: F, t1557: F, t422: F, t173: F, t1743: F, t419: F, t1736: F, t7800: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8052 = t66 * t8051;
    let t8074 = t68 * t391 * t625 * t72;
    let t8076 = t47 * t2247;
    let t8078 = t68 * t8076 * t72;
    let t8079 = 0.70937342644032921812e-2 * t8078;
    let t8086 = t68 * t1675 * t172 * t72;
    let t8088 = t422 * t1557;
    let t8098 = t173 * t1743;
    let t8099 = t419 * t8098;
    let t8101 = t1736 * t7800;
    (t8052, t8074, t8076, t8078, t8079, t8086, t8088, t8099, t8101)
}
