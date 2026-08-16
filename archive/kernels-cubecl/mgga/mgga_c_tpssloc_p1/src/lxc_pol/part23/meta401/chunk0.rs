//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1210/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1210<F: Float>(t10189: F, t5842: F, t5836: F, t5838: F, t698: F, t973: F, t5844: F, t4509: F, t10224: F, t5824: F, t2986: F, t4514: F, t48019: F) -> (F, F, F, F, F, F, F, F) {
    let t61189 = t10189 * t5842;
    let t61250 = t10189 * t5836;
    let t61310 = t973 * t698 * t5838;
    let t61313 = t973 * t698 * t5844;
    let t61322 = t4509 * t5836;
    let t61365 = t4509 * t5842;
    let t61408 = t973 * t10224 * t5824;
    let t61489 = t2986 * t48019 * t4514;
    (t61189, t61250, t61310, t61313, t61322, t61365, t61408, t61489)
}
