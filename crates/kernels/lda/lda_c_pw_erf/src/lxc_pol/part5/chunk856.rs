//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 856/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk856<F: Float>(t7923: F, t7948: F, t2634: F, t774: F, t2642: F, t2610: F, t756: F, t133: F, t1870: F, t1871: F, t3280: F, t3284: F, t3322: F, t3348: F, t5660: F, t7203: F, t7205: F, t7893: F, t7896: F, t7915: F, t7920: F, t7926: F, t7935: F, t7940: F, t7941: F, t7947: F) -> (F, F, F, F, F) {
    let t7949 = t7923 + t7948;
    let t7957 = t2634 * t774;
    let t7960 = t774 * t2642;
    let t7970 = t756 * t2610;
    let t7974 = F::cast_from(1.724255_f64) * t7203 - F::cast_from(5.172765_f64) * t7205 - t3348 - t3284 + t3280 - t7926 - t3322 - t7940 + t7941 - F::cast_from(1.724255_f64) * t133 * t7915 - t7935 + t7947 - F::cast_from(2.2990066666666666_f64) * t5660 - t7893 + t7896 - F::cast_from(20.69106_f64) * t133 * t7920 + F::cast_from(15.518295_f64) * t1870 * t1871 * t7970;
    (t7949, t7957, t7960, t7970, t7974)
}
