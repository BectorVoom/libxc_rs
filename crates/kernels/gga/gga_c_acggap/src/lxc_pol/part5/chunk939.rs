//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 939/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk939<F: Float>(t3409: F, t4713: F, t13084: F, t4921: F, t1494: F, t3570: F, t1498: F, t3431: F, t4708: F, t13087: F, t4904: F, t14220: F, t4425: F, t4741: F, t1163: F, t13889: F, t1540: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17831 = t3409 * t4713;
    let t17837 = t13084 * t4921;
    let t17851 = t3570 * t1494;
    let t17853 = t3570 * t1498;
    let t17855 = t3431 * t4708;
    let t17857 = t13087 * t4904;
    let t17859 = t14220 * t4425;
    let t17861 = t14220 * t4741;
    let t17868 = t1163 * t13889 * t1540;
    (t17831, t17837, t17851, t17853, t17855, t17857, t17859, t17861, t17868)
}
