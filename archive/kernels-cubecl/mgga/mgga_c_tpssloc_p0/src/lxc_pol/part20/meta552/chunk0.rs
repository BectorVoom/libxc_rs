//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2097/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2097<F: Float>(t41362: F, t831: F, t2686: F, t9671: F, t2681: F, t2628: F, t2690: F, t812: F, t2635: F, t9674: F, t2697: F, t9618: F) -> (F, F, F, F, F, F) {
    let t41363 = t41362 * t831;
    let t41365 = t9671 * t2686;
    let t41373 = t9671 * t2681;
    let t41385 = t812 * t2628 * t2690;
    let t41386 = t41385 * t2635;
    let t41395 = t9674 * t2686;
    let t41397 = t2697 * t9618;
    (t41363, t41365, t41373, t41386, t41395, t41397)
}
