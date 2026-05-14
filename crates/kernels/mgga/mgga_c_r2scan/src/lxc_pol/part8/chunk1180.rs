//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1180/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1180<F: Float>(t1819: F, t21115: F, t225: F, t234: F, t21430: F, t748: F, t1836: F, t720: F, t21778: F, t5438: F, t1818: F, t21601: F, t61: F, t2090: F, t4: F, t612: F, t615: F) -> (F, F, F, F, F, F) {
    let t22546 = 0.70178683471615754485e2 * t234 * t1819 * t225 * t21115;
    let t22550 = 0.10526802520742363173e2 * t234 * t748 * t225 * t21430;
    let t22554 = 0.31168546390226634765e3 * t234 * t1836 * t720 * t21430;
    let t22557 = 0.77055573020282513724e1 * t5438 * t21778;
    let t22560 = 0.68445575878594514436e3 * t61 * t1818 * t21601;
    let t22574 = 0.8781774676543209876e-2 * t612 * t615 * t4 * t2090;
    (t22546, t22550, t22554, t22557, t22560, t22574)
}
