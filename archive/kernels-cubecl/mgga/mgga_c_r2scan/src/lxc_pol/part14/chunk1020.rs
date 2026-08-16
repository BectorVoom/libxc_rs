//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1020/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1020<F: Float>(t12285: F, t339: F, t341: F, t1127: F, t2410: F, t1020: F, t3522: F, t3745: F, t839: F, t333: F, t335: F, t337: F) -> (F, F, F, F, F, F, F, F) {
    let t12286 = t339 * t12285;
    let t12288 = t341 * t12285;
    let t12292 = t2410 * t1127;
    let t12294 = t1020 * t3522;
    let t12296 = t839 * t3745;
    let t12298 = t333 * t12285;
    let t12300 = t335 * t12285;
    let t12302 = t337 * t12285;
    (t12286, t12288, t12292, t12294, t12296, t12298, t12300, t12302)
}
