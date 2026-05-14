//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 582/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk582<F: Float>(t1929: F, t1934: F, t231: F, t2470: F, t2475: F, t2477: F, t2482: F, t2501: F, t2503: F, t2507: F, t2509: F, t2511: F, t2660: F, t2530: F, t2534: F, t2538: F, t2542: F, t2546: F, t2548: F, t2552: F, t2556: F, t2560: F, t2564: F, t2568: F, t2569: F) -> (F, F) {
    let t2663 = 8.0 / 3.0 * t1929 + 8.0 / 3.0 * t1934 + 4.0 / 3.0 * t2660 * t231 + t2470 + t2475 + t2477 + t2482 - t2501 + t2503 + t2507 - t2509 - t2511;
    let t2665 = -t2530 - t2534 + t2538 + t2542 + t2546 + t2548 + t2552 + t2556 - t2560 - t2564 - t2568 + t2569;
    (t2663, t2665)
}
