//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 760/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk760<F: Float>(t9: F, t9896: F, t109: F, t992: F, t995: F, t991: F, t2909: F, t993: F, t1000: F, t2888: F, t2880: F) -> (F, F, F, F, F, F) {
    let t9897 = t9 * t9896;
    let t9916 = t109 * t992;
    let t9917 = t9916 * t995;
    let t9918 = t991 * t9917;
    let t9924 = t993 * t2909;
    let t9933 = t2888 * t1000;
    let t9938 = t2880 * t1000;
    (t9897, t9916, t9918, t9924, t9933, t9938)
}
