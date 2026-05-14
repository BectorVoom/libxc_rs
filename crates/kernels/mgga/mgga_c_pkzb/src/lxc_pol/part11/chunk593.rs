//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 593/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk593<F: Float>(t1413: F, t1449: F, t3308: F, t3311: F, t3337: F, t3340: F, t3356: F, t42: F, t430: F, t453: F, t972: F) -> (F, F) {
    let t3359 = 0.165625e-1 * t3308 * t42 - 0.6625e-1 * t1413 * t3311 + 0.165625e-1 * t430 * t3337 + 0.496875e-1 * t1449 * t3340 - 0.165625e-1 * t453 * t3356;
    let t3363 = t972 * t972;
    (t3359, t3363)
}
