//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1041/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1041<F: Float>(t5900: F, t9132: F, t5918: F, t8232: F, t1359: F, t7763: F, t1369: F, t1637: F, t5909: F, t7800: F, t1642: F, t2112: F, t378: F, t9236: F, t1370: F, t7943: F) -> (F, F, F, F, F, F, F, F) {
    let t95293 = t9132 * t5900;
    let t95301 = t8232 * t5918;
    let t95312 = t1359 * t7763;
    let t95330 = t1369 * t1637 * t5909;
    let t95332 = t1359 * t7800;
    let t95340 = t1642 * t2112;
    let t95344 = t378 * t9236;
    let t95368 = t1369 * t7943 * t1370;
    (t95293, t95301, t95312, t95330, t95332, t95340, t95344, t95368)
}
