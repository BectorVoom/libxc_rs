//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 729/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk729<F: Float>(t2176: F, t529: F, t1976: F, t542: F, t519: F, t4801: F, t4803: F, t4806: F, t4809: F, t4812: F, t4815: F, t4817: F, t4822: F, t4824: F, t4828: F, t4833: F, t4836: F, t4840: F, t4845: F, t4847: F) -> (F, F, F, F, F) {
    let t4848 = t2176 * t529;
    let t4849 = t1976 * t542;
    let t4850 = t4848 * t4849;
    let t4852 = 16.0 / 45.0 * t519 * t4850;
    let t4853 = -t4801 - t4803 + t4806 - t4809 - t4812 + t4815 + t4817 - t4822 + t4824 + t4828 + t4833 - t4836 - t4840 + t4845 - t4847 - t4852;
    (t4848, t4849, t4850, t4852, t4853)
}
