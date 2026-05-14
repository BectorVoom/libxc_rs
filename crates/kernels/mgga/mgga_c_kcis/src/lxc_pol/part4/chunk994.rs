//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 994/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk994<F: Float>(t1676: F, t2331: F, t22: F, t4864: F, t4715: F, t13710: F, t13712: F, t13723: F, t13732: F, t13767: F, t13939: F, t13942: F, t9726: F, t9729: F, t13717: F, t13742: F, t13772: F, t13775: F, t13777: F, t13881: F, t13886: F, t13888: F, t13892: F, t13910: F, t13912: F, t13915: F, t13918: F, t13921: F, t13924: F, t13927: F, t13931: F, t13934: F, t9681: F, t9683: F, t9691: F) -> (F, F, F, F) {
    let t13945 = t2331 * t1676;
    let t13948 = t22 * t4864;
    let t13949 = t13948 * t4715;
    let t13951 = 0.13287407407407407408e0 * t13712 - t13939 + 0.11958666666666666667e1 * t13723 - 0.17938e1 * t13732 - t9726 - t9729 + 0.3071625e0 * t13942 + 0.1898925e1 * t13767 - 0.91285185185185185185e-1 * t13945 - 0.13287407407407407408e0 * t13710 + 0.71202444444444444443e0 * t13949;
    let t13953 = 0.142419375e1 * t13772 - 0.76790625e-1 * t13881 - 0.1898925e1 * t13775 - 0.9494625e0 * t13777 + 0.3071625e0 * t13886 + 0.15358125e0 * t13888 - 0.16431333333333333333e0 * t13892 + 0.99655555555555555557e-1 * t9681 + 0.66437037037037037038e-1 * t9683 - 0.26574814814814814816e0 * t9691 + t13910 + 0.36514074074074074074e-1 * t13912 - 0.27385555555555555556e-1 * t13915 - 0.36514074074074074075e-1 * t13918 - 0.10954222222222222222e0 * t13921 + 0.16431333333333333333e0 * t13924 + 0.65725333333333333332e0 * t13927 + 0.21924222222222222222e1 * t13717 + 0.16431333333333333333e0 * t13931 - 0.49293999999999999999e0 * t13934 - 0.59793333333333333334e0 * t13742 + t13951;
    (t13945, t13948, t13949, t13953)
}
