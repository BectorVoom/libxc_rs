//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 493/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk493<F: Float>(t2505: F, t493: F, t795: F, t808: F, t822: F, t835: F, t1371: F, t2411: F, t2415: F, t589: F, t2419: F, t1346: F, t1366: F, t1941: F, t2053: F, t2413: F, t2417: F, t2421: F, t25: F) -> (F, F, F, F, F, F, F) {
    let t2507 = 4.0 / 15.0 * t493 * t2505;
    let t2509 = 4.0 / 15.0 * t795 * t808;
    let t2511 = 4.0 / 15.0 * t822 * t835;
    let t2517 = t1371 * t2411;
    let t2520 = t589 * t2415;
    let t2523 = t589 * t2419;
    let t2526 = t1346 + 0.023994444444444443 * t1941 - 0.023994444444444443 * t2413 + 0.07198333333333333 * t2417 - 0.035991666666666665 * t2421 + t1366 + 0.008888888888888889 * t2053 - 0.0022222222222222222 * t25 * t2517 + 0.013333333333333334 * t25 * t2520 - 0.006666666666666667 * t25 * t2523;
    (t2507, t2509, t2511, t2517, t2520, t2523, t2526)
}
