//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 983/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk983<F: Float>(t14872: F, t287: F, t1471: F, t14759: F, t14728: F, t4092: F, t10364: F, t6: F, t1200: F, t285: F, t230: F, t3750: F, t1234: F, t2789: F, t10696: F, t1240: F) -> (F, F, F, F, F, F, F, F) {
    let t54875 = t14872 * t287;
    let t54891 = t14759 * t1471;
    let t54924 = t4092 * t14728;
    let t54927 = t10364 * t6;
    let t54928 = t1200 * t54927;
    let t55011 = t285 * t54927;
    let t55105 = t230 * t3750;
    let t55543 = t1234 * t2789;
    let t55768 = t1240 * t10696;
    (t54875, t54891, t54924, t54928, t55011, t55105, t55543, t55768)
}
