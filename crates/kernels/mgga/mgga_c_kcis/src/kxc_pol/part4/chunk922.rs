//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 922/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk922<F: Float>(t11913: F, t4166: F, t1401: F, t4036: F, t3754: F, t89: F, t4034: F, t516: F, t1445: F, t4024: F, t4028: F, t532: F, t1444: F, t160: F, t833: F, t2645: F, t4061: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11914 = t11913 * t4166;
    let t11918 = t1401 * t4036;
    let t11920 = t89 * t3754;
    let t11939 = 1.0 / t4034 / t516;
    let t11947 = t1445 * t4024;
    let t11949 = t532 * t4028;
    let t11951 = t160 * t1444;
    let t11952 = t11951 * t833;
    let t11954 = t4061 * t2645;
    (t11914, t11918, t11920, t11939, t11947, t11949, t11951, t11952, t11954)
}
