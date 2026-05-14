//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1177/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1177<F: Float>(t1771: F, t5416: F, t1762: F, t1987: F, t5916: F, t5216: F, t5967: F, t124: F, t1835: F, t1946: F, t5206: F, t5559: F, t1763: F, t5534: F, t1767: F, t5556: F) -> (F, F, F, F, F, F, F) {
    let t22468 = t1771 * t5416;
    let t22472 = 0.25685191006760837908e1 * t1762 * t5916 * t1987;
    let t22473 = t5967 * t5216;
    let t22478 = 0.77055573020282513724e1 * t1762 * t124 * t1835 * t1946;
    let t22481 = 0.23116671906084754117e2 * t1762 * t5206 * t5559;
    let t22484 = 0.1301229756036208781e0 * t1762 * t1763 * t5534;
    let t22487 = 0.39036892681086263432e0 * t1762 * t1767 * t5556;
    (t22468, t22472, t22473, t22478, t22481, t22484, t22487)
}
