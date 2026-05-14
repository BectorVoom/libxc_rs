//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1399/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1399<F: Float>(t22524: F, t22527: F, t897: F, t5714: F, t898: F, t5439: F, t717: F, t21379: F, t21383: F, t21387: F, t21392: F, t21394: F, t21396: F, t21401: F, t21404: F, t22564: F, t229: F, t23893: F, t26481: F, t41: F, t951: F) -> (F,) {
    let t26488 = t22524 * t897 * t22527;
    let t26490 = t898 * t5714;
    let t26493 = t898 * t717 * t5439;
    let t26495 = t21379 + t21383 - t21387 + t21392 + 0.4572795528e-1 * t21394 - 0.1524265176e-1 * t21396 - t21401 + t21404 + 0.5143752e0 * t26481 - 0.675260332e-1 * t951 * t22564 - t41 * t23893 * t229 + 0.16206247968e1 * t26488 - 0.11558335953042377058e2 * t26490 + 0.57791679765211885293e1 * t26493;
    (t26495,)
}
