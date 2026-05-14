//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 879/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk879<F: Float>(t1466: F, t6973: F, t571: F, t2168: F, t4738: F, t2328: F, t529: F, t542: F, t1440: F, t1325: F, t2183: F, t2171: F, t2188: F, t2406: F, t518: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6974 = t1466 * t6973;
    let t6976 = 8.0 / 15.0 * t571 * t6974;
    let t6978 = 8.0 / 15.0 * t4738 * t2168;
    let t6979 = t529 * t2328;
    let t6980 = t6979 * t542;
    let t6981 = t1440 * t6980;
    let t6983 = 4.0 / 15.0 * t1325 * t6981;
    let t6985 = 8.0 / 15.0 * t4738 * t2183;
    let t6987 = 8.0 / 15.0 * t2171 * t2188;
    let t6988 = t2406 * t518;
    (t6974, t6976, t6978, t6979, t6980, t6981, t6983, t6985, t6987, t6988)
}
