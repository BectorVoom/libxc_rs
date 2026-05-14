//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 905/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk905<F: Float>(t1: F, t2363: F, t3: F, t604: F, t5039: F, t5055: F, t5057: F, t5172: F, t6751: F, t6755: F, t6758: F, t6761: F, t6765: F, t6769: F, t6773: F, t6776: F, t6778: F, t6780: F, t6782: F) -> (F, F) {
    let t7266 = t2363 * t1 * t3;
    let t7267 = t7266 * t604;
    let t7269 = -t6751 + t6755 - t6758 - t6761 - t6765 + t6769 - t6773 - t6776 - t6778 - t6780 + 0.10821041362364843 * t7267 + t6782 + t5039 - t5055 - t5057 - t5172;
    (t7266, t7269)
}
