//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1250/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1250<F: Float>(t237: F, t30410: F, t30440: F, t30466: F, t30498: F, t30548: F, t30617: F, t30663: F, t30700: F, t1116: F, t25656: F, t2860: F, t9398: F) -> (F, F, F) {
    let t30704 = t237 * (t30410 + t30440 + t30466 + t30498 + t30548 + t30617 + t30663 + t30700);
    let t30706 = F::cast_from(0.17544670867903938621e1_f64) * t25656 * t1116;
    let t30708 = F::cast_from(0.31168546390226634765e3_f64) * t2860 * t9398;
    (t30704, t30706, t30708)
}
