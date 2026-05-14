//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 796/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk796<F: Float>(t247: F, t4713: F, t251: F, t2252: F, t652: F, t256: F, t19: F, t1904: F, t644: F, t647: F, t1432: F, t850: F, t1427: F, t2260: F, t4464: F, t4466: F, t4468: F, t4470: F, t4471: F, t4472: F, t4473: F, t4474: F, t4478: F, t4482: F, t4486: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5787 = t4713 * t247;
    let t5788 = t5787 * t251;
    let t5791 = t2252 * t652;
    let t5793 = 2.0 / 3.0 * t5791 * t256;
    let t5794 = t1904 * t19;
    let t5795 = t5794 * t644;
    let t5797 = 0.12155555555555556 * t5795 * t647;
    let t5798 = t850 * t1432;
    let t5799 = t5798 * t256;
    let t5801 = t2260 * t1427;
    let t5803 = t5788 * t256 / 3.0 + t5793 + t5797 + t5799 / 3.0 + 0.12155555555555556 * t5801 - t4464 - t4466 + t4468 + t4470 + t4471 + t4472 + t4473 + t4474 - t4478 - t4482 - t4486;
    (t5787, t5788, t5791, t5793, t5794, t5795, t5797, t5798, t5799, t5801, t5803)
}
