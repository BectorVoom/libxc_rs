//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 717/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk717<F: Float>(t166: F, t6595: F, t161: F, t4815: F, t822: F, t137: F, t132: F, t1848: F, t853: F, t2101: F, t831: F, t5349: F, t5354: F, t5356: F, t5363: F, t5369: F, t5370: F, t5372: F, t6586: F, t6588: F, t6590: F, t6592: F, t6594: F) -> (F, F, F, F, F, F, F, F) {
    let t6596 = t166 * t6595;
    let t6598 = t161 * t6596 / F::new(30.0);
    let t6599 = t4815 * t822;
    let t6600 = t137 * t6599;
    let t6602 = t132 * t6600 / F::new(15.0);
    let t6604 = t1848 * t853 / F::new(15.0);
    let t6606 = t831 * t2101 / F::new(15.0);
    let t6607 = -t5349 + t5354 - t5356 - t5363 + t5369 + F::new(8.0) / F::new(9.0) * t5370 - F::new(4.0) / F::new(27.0) * t5372 - t6586 - t6588 - t6590 - t6592 - t6594 - t6598 - t6602 - t6604 - t6606;
    (t6596, t6598, t6599, t6600, t6602, t6604, t6606, t6607)
}
