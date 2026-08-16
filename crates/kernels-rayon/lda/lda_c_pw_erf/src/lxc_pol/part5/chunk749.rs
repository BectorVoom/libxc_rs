//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 749/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk749(t589: f64, t6413: f64, t6446: f64, t6361: f64, t1371: f64, t6366: f64, t6379: f64, t3587: f64, t6384: f64, t6388: f64, t2061: f64, t25: f64, t4657: f64, t4663: f64, t5022: f64, t5024: f64, t6641: f64, t6644: f64, t6647: f64, t6652: f64, t6655: f64, t6660: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6812 = t589 * t6413;
    let t6815 = t589 * t6446;
    let t6818 = t589 * t6361;
    let t6821 = t1371 * t6366;
    let t6824 = t1371 * t6379;
    let t6827 = t3587 * t6384;
    let t6830 = t1371 * t6388;
    let t6842 = -0.023994444444444443_f64 * t6660 - 0.04_f64 * t25 * t6812 - 0.05333333333333334_f64 * t2061 * t6815 + 0.013333333333333334_f64 * t25 * t6818 - 0.0022222222222222222_f64 * t25 * t6821 + 0.013333333333333334_f64 * t25 * t6824 - 0.002962962962962963_f64 * t25 * t6827 + 0.008888888888888889_f64 * t2061 * t6830 + 0.14396666666666666_f64 * t6644 - 0.03999074074074074_f64 * t6641 + 0.09597777777777777_f64 * t6647 - 0.21595_f64 * t6652 - 0.2879333333333333_f64 * t6655 - 0.017777777777777778_f64 * t5022 - 0.014814814814814815_f64 * t5024 - 0.03199259259259259_f64 * t4657 - 0.047988888888888886_f64 * t4663;
    (t6812, t6815, t6818, t6821, t6824, t6827, t6830, t6842)
}
