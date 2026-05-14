//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 714/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk714<F: Float>(t4693: F, t557: F, t11: F, t213: F, t558: F, t174: F, t3540: F, t3627: F, t3629: F, t3631: F, t3646: F, t4013: F, t4657: F, t4659: F, t4662: F, t4663: F, t4668: F, t4673: F, t4678: F, t4682: F, t4686: F, t4691: F) -> (F, F, F, F, F) {
    let t4694 = t557 * t4693;
    let t4695 = t11 * t4694;
    let t4697 = t213 * t558;
    let t4699 = t174 * t3540 * t4697;
    let t4701 = t4013 + 0.0016792592592592592 * t3627 - 0.0004198148148148148 * t3631 + 0.0012594444444444445 * t3646 - 0.0006297222222222223 * t3629 + 0.0008396296296296296 * t4657 - 0.0008396296296296296 * t4659 + t4662 + 0.01385388888888889 * t4663 + 0.002099074074074074 * t4668 - 0.007556666666666666 * t4673 - 0.005037777777777778 * t4678 + 0.0012594444444444445 * t4682 + 0.011335 * t4686 + 0.015113333333333333 * t4691 - 0.003778333333333333 * t4695 - 0.003778333333333333 * t4699;
    (t4694, t4695, t4697, t4699, t4701)
}
