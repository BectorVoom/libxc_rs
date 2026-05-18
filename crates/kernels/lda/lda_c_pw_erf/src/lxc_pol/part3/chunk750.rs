//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 750/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk750<F: Float>(t4818: F, t951: F, t3832: F, t571: F, t2027: F, t3794: F, t789: F, t944: F, t1326: F, t1325: F, t197: F, t2176: F) -> (F, F, F, F, F, F, F, F) {
    let t4819 = t4818 * t951;
    let t4820 = t3832 * t4819;
    let t4822 = F::new(4.0) / F::new(27.0) * t571 * t4820;
    let t4824 = F::new(16.0) / F::new(45.0) * t3794 * t2027;
    let t4825 = t789 * t944;
    let t4826 = t1326 * t4825;
    let t4828 = F::new(8.0) / F::new(45.0) * t1325 * t4826;
    let t4829 = t2176 * t197;
    (t4819, t4820, t4822, t4824, t4825, t4826, t4828, t4829)
}
