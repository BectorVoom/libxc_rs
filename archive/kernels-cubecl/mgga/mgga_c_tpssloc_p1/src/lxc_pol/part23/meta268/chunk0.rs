//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 944/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk944<F: Float>(t19871: F, t3805: F, t6394: F, t19956: F, t550: F, t6347: F, t5249: F, t1799: F, t3792: F, t6414: F) -> (F, F, F, F, F, F, F) {
    let t20454 = t3805 * t19871 * t6394;
    let t20460 = t3805 * t19956 * t6394;
    let t20463 = t550 * t6347;
    let t20465 = t3805 * t5249 * t20463;
    let t20468 = t3792 * t1799;
    let t20470 = t3805 * t19871 * t20468;
    let t20473 = t3792 * t6414;
    (t20454, t20460, t20463, t20465, t20468, t20470, t20473)
}
