//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 629/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk629<F: Float>(t2281: F, t668: F, t1901: F, t646: F, t2253: F, t656: F, t2256: F, t1410: F, t851: F, t168: F, t2292: F, t635: F, t1905: F, t632: F, t1143: F, t781: F) -> (F, F, F, F, F, F, F, F) {
    let t5837 = 4.0 / 45.0 * t2281 * t668;
    let t5859 = t1901 * t646;
    let t5871 = 4.0 / 9.0 * t2253 * t656;
    let t5872 = t2256 * t656;
    let t5874 = t851 * t1410;
    let t5887 = 0.039794582218349216 * t168 * t635 * t2292;
    let t5891 = 0.1675256410710088 * t1905 * t632;
    let t5892 = t781 * t1143;
    (t5837, t5859, t5871, t5872, t5874, t5887, t5891, t5892)
}
