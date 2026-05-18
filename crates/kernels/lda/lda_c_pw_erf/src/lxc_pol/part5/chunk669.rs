//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 669/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk669<F: Float>(t34: F, t92: F, t93: F, t2281: F, t668: F, t1901: F, t646: F, t2253: F, t656: F, t2256: F, t1410: F, t851: F) -> (F, F, F, F, F, F, F) {
    let t5812 = t92 * t34;
    let t5823 = t93 * t34;
    let t5837 = F::new(4.0) / F::new(45.0) * t2281 * t668;
    let t5859 = t1901 * t646;
    let t5871 = F::new(4.0) / F::new(9.0) * t2253 * t656;
    let t5872 = t2256 * t656;
    let t5874 = t851 * t1410;
    (t5812, t5823, t5837, t5859, t5871, t5872, t5874)
}
