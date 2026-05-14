//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 557/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk557<F: Float>(t2610: F, t8392: F, t9698: F, t1882: F, t2591: F, t2596: F, t726: F, t8232: F, t2587: F, t2614: F, t2581: F, t2542: F, t761: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10090 = t8392 * t2610;
    let t10119 = 28.0 / 27.0 * t9698;
    let t10126 = t1882 * t2591;
    let t10128 = t1882 * t2596;
    let t10134 = t8232 * t726;
    let t10140 = t1882 * t2587;
    let t10146 = t1882 * t2614;
    let t10148 = t1882 * t2581;
    let t10153 = t2542 * t761;
    (t10090, t10119, t10126, t10128, t10134, t10140, t10146, t10148, t10153)
}
