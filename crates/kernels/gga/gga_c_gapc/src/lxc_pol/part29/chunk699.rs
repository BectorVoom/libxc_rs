//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 699/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk699<F: Float>(t314: F, t7974: F, t2387: F, t825: F, t2389: F, t2763: F, t327: F, t966: F, t286: F, t875: F) -> (F, F, F, F, F, F) {
    let t7975 = t7974 * t314;
    let t8061 = t2387 * t825;
    let t8117 = t2389 * t825;
    let t8131 = t327 * t2763;
    let t8132 = t8131 * t966;
    let t8133 = t875 * t286;
    (t7975, t8061, t8117, t8131, t8132, t8133)
}
