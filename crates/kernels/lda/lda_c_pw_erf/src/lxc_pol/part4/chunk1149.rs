//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1149/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1149<F: Float>(t1405: F, t2425: F, t4568: F, t6209: F, t1394: F, t2407: F, t2127: F, t6580: F, t2131: F, t12498: F, t2098: F, t1391: F, t185: F, t186: F, t1524: F, t2468: F) -> (F, F, F, F, F, F, F, F) {
    let t16917 = 4.0 / 15.0 * t2425 * t1405;
    let t16918 = t6209 * t4568;
    let t16919 = 8.0 / 9.0 * t16918;
    let t16921 = 8.0 / 15.0 * t2407 * t1394;
    let t16922 = t6580 * t2127;
    let t16923 = 32.0 / 45.0 * t16922;
    let t16925 = 16.0 / 15.0 * t6580 * t2131;
    let t16926 = 16.0 / 45.0 * t12498;
    let t16927 = t2098 * t2098;
    let t16931 = 8.0 / 15.0 * t185 * t186 * t1391 * t16927;
    let t16933 = 8.0 / 15.0 * t1524 * t2468;
    (t16917, t16919, t16921, t16923, t16925, t16926, t16931, t16933)
}
