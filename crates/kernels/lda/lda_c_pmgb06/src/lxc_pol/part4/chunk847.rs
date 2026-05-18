//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 847/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk847<F: Float>(t2247: F, t2249: F, t5858: F, t3643: F, t5749: F, t5752: F, t5755: F, t5762: F, t5785: F, t5789: F, t5801: F, t5804: F, t5810: F, t5852: F, t5855: F, t69: F) -> (F, F) {
    let t5860 = t2247 * t5858 * t2249;
    let t5862 = t5749 + t5752 - t5755 - t3643 - F::new(0.7663355555555555) * t5852 + t5855 - F::new(1.724255) * t69 * t5810 - t5762 - t5785 - F::new(3.44851) * t5860 - t5789 + t5801 + t5804;
    (t5860, t5862)
}
