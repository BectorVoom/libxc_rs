//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 555/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk555<F: Float>(t376: F, t5921: F, t89: F, t1882: F, t5886: F, t5866: F, t5875: F, t1366: F, t8232: F, t1378: F, t2178: F) -> (F, F, F, F, F, F, F) {
    let t23930 = t376 * t5921;
    let t23931 = t89 * t23930;
    let t23943 = t1882 * t5886;
    let t23945 = t1882 * t5866;
    let t23947 = t1882 * t5875;
    let t23950 = 4.0 / 27.0 * t8232 * t1366;
    let t23997 = t1378 * t2178;
    (t23930, t23931, t23943, t23945, t23947, t23950, t23997)
}
