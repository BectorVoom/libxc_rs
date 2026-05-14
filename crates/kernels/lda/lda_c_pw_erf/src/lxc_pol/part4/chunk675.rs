//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 675/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk675<F: Float>(t4383: F, t87: F, t40: F, t2705: F, t1081: F, t1772: F, t1051: F, t1765: F, t1055: F, t1798: F, t75: F, t402: F, t2740: F, t4: F, t748: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4384 = t4383 * t87;
    let t4385 = t40 * t4384;
    let t4386 = 0.021687161765563047 * t2705;
    let t4387 = t1772 * t1081;
    let t4388 = 0.0002441540671567088 * t4387;
    let t4389 = t1765 * t1051;
    let t4390 = 0.5848223397455204 * t4389;
    let t4391 = t1765 * t1055;
    let t4392 = 17.315755899375862 * t4391;
    let t4393 = t1798 * t75;
    let t4394 = t4393 * t402;
    let t4395 = 1.169644679491041 * t4394;
    let t4396 = 1.169644679491041 * t2740;
    let t4397 = t748 * t4;
    (t4384, t4385, t4386, t4387, t4388, t4389, t4390, t4391, t4392, t4393, t4394, t4395, t4396, t4397)
}
