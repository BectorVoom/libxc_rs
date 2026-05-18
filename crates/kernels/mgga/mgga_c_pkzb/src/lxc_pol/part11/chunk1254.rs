//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1254/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1254<F: Float>(t2860: F, t9205: F, t2870: F, t9242: F, t10949: F, t1987: F, t10956: F, t9352: F, t10960: F, t7299: F, t730: F, t9397: F) -> (F, F, F, F, F, F, F) {
    let t30747 = F::new(0.17544670867903938621e1) * t2860 * t9205;
    let t30749 = F::new(0.17544670867903938621e1) * t9242 * t2870;
    let t30751 = F::new(0.35089341735807877242e1) * t1987 * t10949;
    let t30753 = F::new(0.10254018858216406658e4) * t1987 * t10956;
    let t30755 = F::new(0.51947577317044391276e2) * t2860 * t9352;
    let t30758 = F::new(0.5848223622634646207e0) * t1987 * t10960;
    let t30761 = F::new(0.31168546390226634765e3) * t730 * t9397 * t7299;
    (t30747, t30749, t30751, t30753, t30755, t30758, t30761)
}
