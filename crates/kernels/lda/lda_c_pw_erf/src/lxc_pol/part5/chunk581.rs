//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 581/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk581<F: Float>(t34: F, t950: F, t1081: F, t1772: F, t1051: F, t1765: F, t1055: F, t1798: F, t75: F, t402: F, t4: F, t748: F) -> (F, F, F, F, F, F, F, F) {
    let t4370 = t950 * t34;
    let t4387 = t1772 * t1081;
    let t4389 = t1765 * t1051;
    let t4391 = t1765 * t1055;
    let t4393 = t1798 * t75;
    let t4394 = t4393 * t402;
    let t4395 = 1.169644679491041 * t4394;
    let t4397 = t748 * t4;
    (t4370, t4387, t4389, t4391, t4393, t4394, t4395, t4397)
}
