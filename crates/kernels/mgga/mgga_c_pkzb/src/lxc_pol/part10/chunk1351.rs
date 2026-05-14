//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1351/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1351<F: Float>(t3152: F, t8099: F, t898: F, t3157: F, t8028: F, t3932: F, t6362: F, t3282: F, t10282: F, t914: F, t1227: F, t3246: F, t3874: F, t937: F, t10334: F, t6455: F) -> (F, F, F, F, F, F, F, F) {
    let t26898 = 0.23392894490538584828e1 * t898 * t3152 * t8099;
    let t26900 = 0.23392894490538584828e1 * t8028 * t3157;
    let t26901 = t3932 * t6362;
    let t26905 = t3282 * t3282;
    let t26927 = t914 * t10282;
    let t26936 = t3246 * t1227;
    let t26940 = t937 * t3874;
    let t26948 = t6455 * t10334;
    (t26898, t26900, t26901, t26905, t26927, t26936, t26940, t26948)
}
