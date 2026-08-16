//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1028/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1028<F: Float>(t11365: F, t898: F, t11143: F, t11231: F, t11236: F, t11292: F, t11295: F, t11318: F, t11355: F, t11357: F, t11359: F, t11361: F, t11363: F) -> (F, F) {
    let t11367 = F::cast_from(0.35089341735807877242e1_f64) * t898 * t11365;
    let t11368 = -t11355 - t11357 - t11359 - t11361 + t11363 + t11318 - t11292 + t11295 - t11231 + t11236 - t11367 + t11143;
    (t11367, t11368)
}
