//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1138/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1138<F: Float>(t10949: F, t1987: F, t10956: F, t2860: F, t9352: F, t10960: F, t7299: F, t730: F, t9397: F, t3605: F, t7527: F, t25671: F, t2852: F, t3618: F, t7560: F, t9351: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30751 = 0.35089341735807877242e1 * t1987 * t10949;
    let t30753 = 0.10254018858216406658e4 * t1987 * t10956;
    let t30755 = 0.51947577317044391276e2 * t2860 * t9352;
    let t30758 = 0.5848223622634646207e0 * t1987 * t10960;
    let t30761 = 0.31168546390226634765e3 * t730 * t9397 * t7299;
    let t30764 = 0.35089341735807877242e1 * t730 * t7527 * t3605;
    let t30767 = 0.51947577317044391277e2 * t730 * t25671 * t2852;
    let t30769 = 0.35089341735807877242e1 * t7560 * t3618;
    let t30772 = 0.51947577317044391277e2 * t730 * t9351 * t7299;
    (t30751, t30753, t30755, t30758, t30761, t30764, t30767, t30769, t30772)
}
