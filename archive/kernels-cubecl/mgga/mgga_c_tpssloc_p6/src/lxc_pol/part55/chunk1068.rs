//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1068/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1068<F: Float>(t1873: F, t27888: F, t6534: F, t7266: F, t1874: F, t24932: F, t6525: F, t2314: F, t8675: F, t4034: F, t7408: F, t652: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31885 = t27888 * t1873;
    let t31887 = t7266 * t6534;
    let t31898 = t24932 * t1874;
    let t31900 = t27888 * t1874;
    let t31902 = t7266 * t6525;
    let t31904 = t2314 * t8675;
    let t31906 = t4034 * t8675;
    let t31908 = t7408 * t1873;
    let t31909 = t652 * t31908;
    (t31885, t31887, t31898, t31900, t31902, t31904, t31906, t31908, t31909)
}
