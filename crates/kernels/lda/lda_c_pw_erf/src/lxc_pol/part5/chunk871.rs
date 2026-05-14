//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 871/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk871<F: Float>(t10527: F, t219: F, t10605: F, t1944: F, t571: F, t9408: F, t10162: F, t1325: F, t2167: F, t1519: F, t1982: F, t1518: F, t2066: F, t211: F, t1131: F, t485: F, t5474: F) -> (F, F, F, F, F, F, F) {
    let t14240 = t10527 * t219;
    let t14255 = t571 * t10605 * t219 * t1944;
    let t14256 = 8.0 / 81.0 * t14255;
    let t14257 = t9408 * t219;
    let t14313 = t1325 * t10162 * t2167;
    let t14314 = 8.0 / 45.0 * t14313;
    let t14351 = t1982 * t1519;
    let t14352 = 4.0 / 45.0 * t14351;
    let t14365 = t211 * t1518 * t2066;
    let t14366 = 4.0 / 45.0 * t14365;
    let t14385 = t5474 * t1131 * t485;
    (t14240, t14256, t14257, t14314, t14352, t14366, t14385)
}
