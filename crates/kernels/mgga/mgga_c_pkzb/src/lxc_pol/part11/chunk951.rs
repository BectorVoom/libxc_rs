//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 951/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk951<F: Float>(t11213: F, t881: F, t890: F, t898: F, t3147: F, t3841: F, t1217: F, t9762: F, t3837: F, t3833: F, t11180: F, t2317: F, t11143: F, t11231: F, t11236: F, t11292: F, t11295: F, t11318: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11353 = t881 * t11213 * t890;
    let t11355 = 0.5848223622634646207e0 * t898 * t11353;
    let t11357 = 0.51947577317044391276e2 * t3147 * t3841;
    let t11359 = 0.17544670867903938621e1 * t9762 * t1217;
    let t11361 = 0.17544670867903938621e1 * t3147 * t3837;
    let t11363 = 0.35089341735807877242e1 * t3147 * t3833;
    let t11365 = t2317 * t11180 * t890;
    let t11367 = 0.35089341735807877242e1 * t898 * t11365;
    let t11368 = -t11355 - t11357 - t11359 - t11361 + t11363 + t11318 - t11292 + t11295 - t11231 + t11236 - t11367 + t11143;
    (t11353, t11355, t11357, t11359, t11361, t11363, t11365, t11367, t11368)
}
