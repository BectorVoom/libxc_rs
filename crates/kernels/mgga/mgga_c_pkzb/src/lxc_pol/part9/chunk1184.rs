//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1184/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1184<F: Float>(t1987: F, t7568: F, t237: F, t5838: F, t1971: F, t721: F, t2852: F, t2149: F, t803: F, t7555: F, t2860: F, t5809: F) -> (F, F, F, F, F, F) {
    let t20636 = F::new(0.17544670867903938621e1) * t1987 * t7568;
    let t20637 = t237 * t5838;
    let t20638 = t1971 * t721;
    let t20641 = F::new(0.31168546390226634765e3) * t20637 * t2852 * t20638;
    let t20642 = t2149 * t803;
    let t20647 = F::new(0.35089341735807877242e1) * t1987 * t7555;
    let t20649 = F::new(0.35089341735807877242e1) * t2860 * t5809;
    (t20636, t20638, t20641, t20642, t20647, t20649)
}
