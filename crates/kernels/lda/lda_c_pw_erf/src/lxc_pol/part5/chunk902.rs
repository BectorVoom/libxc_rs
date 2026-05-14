//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 902/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk902<F: Float>(t12064: F, t6737: F, t581: F, t6843: F, t1294: F, t2425: F, t4568: F, t6209: F, t2127: F, t6580: F, t519: F, t6347: F, t9723: F, t1318: F, t3854: F, t6255: F) -> (F, F, F, F, F, F, F) {
    let t16876 = t12064 * t6737;
    let t16907 = t581 * t6843;
    let t16912 = t2425 * t1294;
    let t16918 = t6209 * t4568;
    let t16922 = t6580 * t2127;
    let t16935 = t519 * t9723 * t6347;
    let t16949 = t1318 * t3854 * t6255;
    (t16876, t16907, t16912, t16918, t16922, t16935, t16949)
}
