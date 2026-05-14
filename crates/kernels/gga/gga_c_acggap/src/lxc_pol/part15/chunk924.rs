//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 924/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk924<F: Float>(t2310: F, t7630: F, t31849: F, t30248: F, t542: F, t1967: F, t8855: F, t31773: F, t8916: F, t7447: F, t8920: F, t1439: F, t1983: F, t7380: F, t1460: F, t1992: F, t2095: F) -> (F, F, F, F, F, F, F, F) {
    let t36333 = t7630 * t2310;
    let t36340 = 0.15724046144802076034e-2 * t31849;
    let t36349 = t30248 * t542;
    let t36351 = t1967 * t8855;
    let t36353 = t31773 * t8916;
    let t36355 = t7447 * t8920;
    let t36364 = t7380 * t1983 * t1439;
    let t36367 = t2095 * t1992 * t1460;
    (t36333, t36340, t36349, t36351, t36353, t36355, t36364, t36367)
}
