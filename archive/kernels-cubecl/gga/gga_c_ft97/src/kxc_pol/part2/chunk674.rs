//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 674/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk674<F: Float>(t1882: F, t2591: F, t2596: F, t726: F, t8232: F, t2587: F, t2614: F, t2581: F, t2542: F, t761: F, t192: F, t7514: F) -> (F, F, F, F, F, F, F, F) {
    let t10126 = t1882 * t2591;
    let t10128 = t1882 * t2596;
    let t10134 = t8232 * t726;
    let t10140 = t1882 * t2587;
    let t10146 = t1882 * t2614;
    let t10148 = t1882 * t2581;
    let t10153 = t2542 * t761;
    let t10157 = t192 * t7514;
    (t10126, t10128, t10134, t10140, t10146, t10148, t10153, t10157)
}
