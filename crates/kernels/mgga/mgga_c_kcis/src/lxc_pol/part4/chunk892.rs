//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 892/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk892<F: Float>(t9909: F, t991: F, t109: F, t992: F, t995: F, t2909: F, t993: F, t1000: F, t2888: F, t2880: F, t2895: F, t2904: F, t24: F, t2887: F, t2890: F, t2877: F, t984: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9910 = t991 * t9909;
    let t9916 = t109 * t992;
    let t9917 = t9916 * t995;
    let t9918 = t991 * t9917;
    let t9924 = t993 * t2909;
    let t9933 = t2888 * t1000;
    let t9938 = t2880 * t1000;
    let t9939 = t9938 * t2895;
    let t9940 = t991 * t9939;
    let t9956 = t2880 * t2904;
    let t9957 = t991 * t9956;
    let t9959 = t24 * t2887;
    let t9960 = t9959 * t2890;
    let t9961 = t991 * t9960;
    let t9970 = t984 * t2877;
    (t9910, t9916, t9918, t9924, t9933, t9938, t9940, t9957, t9959, t9961, t9970)
}
