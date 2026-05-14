//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 523/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk523<F: Float>(t1882: F, t6137: F, t6061: F, t668: F, t1424: F, t2360: F, t1434: F, t1435: F, t2399: F, t1433: F, t458: F) -> (F, F, F, F, F, F) {
    let t24524 = t1882 * t6137;
    let t24526 = t6061 * t668;
    let t24531 = t1424 * t2360;
    let t24537 = t1434 * t2399 * t1435;
    let t24538 = 2.0 / 9.0 * t24537;
    let t24543 = t1433 * t458;
    (t24524, t24526, t24531, t24537, t24538, t24543)
}
