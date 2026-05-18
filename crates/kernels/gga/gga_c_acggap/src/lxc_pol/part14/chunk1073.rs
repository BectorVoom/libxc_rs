//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1073/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1073<F: Float>(t368: F, t5659: F, t7380: F, t1795: F, t355: F, t1083: F, t2095: F, t7839: F, t9593: F, t1165: F, t2068: F, t38837: F, t8600: F) -> (F, F, F, F, F, F, F) {
    let t38889 = t368 * t5659;
    let t38890 = t7380 * t38889;
    let t38892 = t355 * t1795;
    let t38893 = t1083 * t38892;
    let t38894 = t2095 * t38893;
    let t38899 = t7839 * t9593;
    let t38903 = t2068 * t1165 * t8600 * t38837;
    (t38889, t38890, t38892, t38893, t38894, t38899, t38903)
}
