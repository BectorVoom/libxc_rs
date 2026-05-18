//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1003/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1003<F: Float>(t325: F, t6501: F, t4606: F, t6535: F, t6538: F, t6541: F, t1251: F, t5992: F, t1245: F, t2430: F, t925: F, t518: F, t6874: F) -> (F, F, F, F, F, F, F, F) {
    let t15836 = t325 * t6501;
    let t15838 = t4606 * t6535;
    let t15848 = t325 * t6538;
    let t15850 = t325 * t6541;
    let t15852 = t1251 * t5992;
    let t15867 = t1245 * t5992;
    let t15887 = t925 * t2430;
    let t15926 = t6874 * t518;
    (t15836, t15838, t15848, t15850, t15852, t15867, t15887, t15926)
}
