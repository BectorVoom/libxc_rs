//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 430/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk430<F: Float>(t2076: F, t551: F, t595: F, t822: F, t331: F, t803: F, t1268: F, t1967: F, t1972: F, t538: F, t1976: F, t1240: F, t1241: F, t1263: F, t1264: F, t1964: F, t1969: F, t1974: F, t1978: F, t2061: F, t25: F) -> (F, F, F, F, F, F, F) {
    let t2078 = 4.0 / 15.0 * t2076 * t551;
    let t2080 = 2.0 / 15.0 * t822 * t595;
    let t2087 = t331 * t803;
    let t2089 = t1268 * t1967;
    let t2092 = t538 * t1972;
    let t2095 = t538 * t1976;
    let t2098 = t1240 + 0.011997222222222222 * t1241 + 0.011997222222222222 * t1964 - 0.023994444444444443 * t1969 + 0.07198333333333333 * t1974 - 0.07198333333333333 * t1978 + t1263 + 0.0044444444444444444 * t1264 + 0.0044444444444444444 * t2087 - 0.0022222222222222222 * t25 * t2089 + 0.013333333333333334 * t25 * t2092 - 0.013333333333333334 * t2061 * t2095;
    (t2078, t2080, t2087, t2089, t2092, t2095, t2098)
}
