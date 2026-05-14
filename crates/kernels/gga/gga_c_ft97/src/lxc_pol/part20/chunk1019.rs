//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1019/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1019<F: Float>(t6189: F, t8232: F, t1882: F, t24781: F, t2399: F, t6150: F, t89: F, t24827: F, t24785: F, t24838: F, t24778: F, t24845: F, t2567: F, t6187: F, t24665: F, t8392: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t97872 = t8232 * t6189;
    let t97879 = t1882 * t24781;
    let t97889 = t89 * t2399 * t6150;
    let t97895 = t1882 * t24827;
    let t97897 = t1882 * t24785;
    let t97899 = t1882 * t24838;
    let t97917 = t1882 * t24778;
    let t97919 = t1882 * t24845;
    let t97928 = t2567 * t6187;
    let t97933 = t8392 * t24665;
    (t97872, t97879, t97889, t97895, t97897, t97899, t97917, t97919, t97928, t97933)
}
